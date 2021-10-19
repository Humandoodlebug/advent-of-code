module AdventOfCode2020.Day19

open System.IO

type Rule =
    | Letter of char
    | Sequence of int list
    | Dichotomy of int list * int list

let input () =
    let lines =
        File.ReadAllLines "input/day19.txt"
        |> Array.toList

    let rules = List.takeWhile ((<>) "") lines

    let messages =
        List.skipWhile ((<>) "") lines
        |> List.skip 1
        |> List.map (fun x -> x.ToCharArray() |> Array.toList)

    let parseRule (str: string) =
        let [| num'; rule |] = str.Split(": ")
        let num = int num'

        if rule.StartsWith('"') then
            num, Letter(rule.[1])
        elif rule.Contains('|') then
            let [| left; right |] = rule.Split(" | ")
            let leftParts = left.Split(' ') |> Array.toList
            let rightParts = right.Split(' ') |> Array.toList
            let leftInts = List.map int leftParts
            let rightInts = List.map int rightParts
            num, Dichotomy(leftInts, rightInts)
        else
            let parts = rule.Split(' ') |> Array.toList
            let ints = List.map int parts
            num, Sequence ints

    let parsedRules = List.map parseRule rules |> Map.ofList
    (parsedRules, messages)

let rec foldConcat f state xs =
    match xs with
    | [] -> state
    | x :: xs' -> foldConcat f (Seq.collect (f x) state) xs'


let (<|>) f g x = Seq.append (f x) (g x)

let parseMessage rules message =
    let parseLetter l =
        function
        | (c :: cs) when c = l -> Seq.singleton cs
        | _ -> Seq.empty

    let rec parseSequence xs cs =
        foldConcat parseRule (Seq.singleton cs) xs

    and parseDichotomy xs ys = parseSequence xs <|> parseSequence ys

    and parseRule i cs =
        match Map.find i rules with
        | Letter c -> parseLetter c cs
        | Sequence xs -> parseSequence xs cs
        | Dichotomy (xs, ys) -> parseDichotomy xs ys cs

    parseRule 0 message |> Seq.contains []

let run () =
    let rules, messages = input ()

    let validMessages =
        List.filter (parseMessage rules) messages

    let answer = List.length validMessages
    printfn "Number of valid messages: %i" answer

    let rules2 =
        Map.add 8 (Dichotomy([ 42 ], [ 42; 8 ])) rules
        |> Map.add 11 (Dichotomy([ 42; 31 ], [ 42; 11; 31 ]))

    let validMessages2 =
        List.filter (parseMessage rules2) messages

    let answer2 = List.length validMessages2
    printfn "Number of valid messages after rule change: %i" answer2
