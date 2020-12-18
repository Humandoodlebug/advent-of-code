module AdventOfCode2020.Day18

open System
open System.IO

type Symbol =
    | Add
    | Mult
    | OpenParen
    | CloseParen
    | Value of int64

let input () =
    let lines =
        File.ReadAllLines "input/day18.txt"
        |> Array.toList
        |> List.map (fun x -> x.ToCharArray() |> Array.toList)

    let rec parseNum cs =
        match cs with
        | [] -> ([], [])
        | c :: cs' when c >= '0' && c <= '9' ->
            let (xs, cs'') = parseNum cs'
            (c :: xs, cs'')
        | _ -> ([], cs)

    let nextSymbol cs =
        match cs with
        | ' ' :: '+' :: ' ' :: cs' -> Some(Add, cs')
        | ' ' :: '*' :: ' ' :: cs' -> Some(Mult, cs')
        | '(' :: cs' -> Some(OpenParen, cs')
        | ')' :: cs' -> Some(CloseParen, cs')
        | _ :: _ ->
            let num, cs' = parseNum cs
            Some(Value(int64 <| String(Array.ofList num)), cs')
        | _ -> None

    List.map (List.unfold nextSymbol) lines

let rec repeatUntilNone f x =
    match f x with
    | None -> x
    | Some x' -> repeatUntilNone f x'

let eval symbols =
    let rec eval' (s :: ss) =
        let evalNum s ss =
            match s with
            | Value x -> x, ss
            | OpenParen ->
                let x, ss' = eval' ss
                x, ss'
            | sym -> failwith $"Expected {nameof Value} or {nameof OpenParen} but got %A{sym}"

        let evalOp (acc, symbols) =
            match symbols with
            | Add :: s :: ss ->
                let x, ss' = evalNum s ss
                Some(acc + x, ss')
            | Mult :: s :: ss ->
                let x, ss' = evalNum s ss
                Some(acc * x, ss')
            | CloseParen :: _ -> None
            | [] -> None
            | sym -> failwith $"Expected an operator or {nameof CloseParen} but got %A{sym}"

        let initNum, ss' = evalNum s ss
        let finalNum, ss'' = repeatUntilNone evalOp (initNum, ss')
        if ss''.Length > 0 then finalNum, ss''.Tail else finalNum, ss''

    eval' symbols |> fst

let eval2 symbols =
    let rec eval' (s :: ss) =
        let evalNum s ss =
            match s with
            | Value x -> x, ss
            | OpenParen ->
                let x, ss' = eval' ss
                x, ss'
            | sym -> failwith $"Expected {nameof Value} or {nameof OpenParen} but got %A{sym}"

        let evalOpLower (acc, symbols) =
            match symbols with
            | Add :: s :: ss ->
                let x, ss' = evalNum s ss
                Some(acc + x, ss')
            | _ -> None

        let evalOp (acc, symbols) =
            match symbols with
            | Add :: s :: ss ->
                let x, ss' = evalNum s ss
                Some(acc + x, ss')
            | Mult :: s :: ss ->
                let x, ss' = evalNum s ss
                let x', ss'' = repeatUntilNone evalOpLower (x, ss')
                Some(acc * x', ss'')
            | CloseParen :: _ -> None
            | [] -> None
            | sym -> failwith $"Expected an operator or {nameof CloseParen} but got %A{sym}"

        let initNum, ss' = evalNum s ss
        let finalNum, ss'' = repeatUntilNone evalOp (initNum, ss')
        if ss''.Length > 0 then finalNum, ss''.Tail else finalNum, ss''

    eval' symbols |> fst

let run () =
    let inp = input ()
    let result = List.sumBy eval inp
    printfn "Sum of results: %i" result
    let result2 = List.sumBy eval2 inp
    printfn "Sum of results with inverse-precedence: %i" result2
