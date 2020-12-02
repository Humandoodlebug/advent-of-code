module AdventOfCode2020.Day1

open System

let input () =
    let extract =
        function
        | [| x |] -> x
        | _ -> raise <| ArgumentException()

    Util.readFileInts "input/day1.txt"
    |> Seq.map extract
    |> Seq.toList

let findSumTo sum inp =
    let sortedInput = List.sort inp
    let reversedSortedInput = List.rev sortedInput

    let rec findSumTo' l1 l2 =
        match l1, l2 with
        | [], _ -> None
        | _, [] -> None
        | x1 :: inp1, x2 :: inp2 ->
            match x1 + x2 with
            | v when v = sum -> Some(x1, x2)
            | v when v > sum -> findSumTo' (x1 :: inp1) inp2
            | v when v < sum -> findSumTo' inp1 (x2 :: inp2)
            | _ -> raise <| Exception()

    findSumTo' sortedInput reversedSortedInput

let findSumTo3 sum inp =
    let isSome =
        function
        | Some _ -> true
        | None -> false

    let mapAnswer =
        function
        | (x1, Some (x2, x3)) -> Some(x1, x2, x3)
        | (_, None) -> None

    let answer =
        Seq.map (fun x -> x, findSumTo (sum - x) inp) inp
        |> Seq.map mapAnswer
        |> Seq.tryFind isSome

    match answer with
    | Some x -> x
    | None -> None

let run () =
    let inp = input ()
    let (Some (x1, x2)) = Util.time (fun () -> findSumTo 2020L inp)
    printfn "%i + %i = 2020" x1 x2
    printfn "%i * %i = %i" x1 x2 (x1 * x2)
    let (Some (x1, x2, x3)) = Util.time (fun () -> findSumTo3 2020L inp)
    printfn "%i + %i + %i = 2020" x1 x2 x3
    printfn "%i * %i * %i = %i" x1 x2 x3 (x1 * x2 * x3)
