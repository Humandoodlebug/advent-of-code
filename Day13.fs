module AdventOfCode2020.Day13

open System
open System.IO

type Bus =
    | Num of int64
    | X

let input () =
    let [| l1; l2 |] = File.ReadAllLines "input/day13.txt"
    let arrival = int64 l1
    let rest = l2.Split(',') |> Array.toList

    let interpret =
        function
        | "x" -> X
        | i -> Num <| int64 i

    (arrival, List.map interpret rest)

let interpretBusses arrival busses =
    List.minBy (fun x -> x - arrival % x) busses

/// Chinese Remainder Theorem
let crt rems mods =
    let N = List.reduce (*) mods
    let Ns = List.map (fun n -> N / n) mods

    /// Modulo Inverse
    let mi x m =
        let Nim = x % m
        Seq.initInfinite (int64 >> (+) 1L)
        |> Seq.find (fun z -> (Nim * z) % m = 1L)

    let xs = List.map2 mi Ns mods

    let ms =
        List.map3 (fun a b c -> a * b * c) rems Ns xs

    let sum = List.reduce (+) ms
    sum % N

let run () =
    let (arrival, busses) = input ()

    let getNum =
        function
        | X -> None
        | Num x -> Some x

    let justNums = List.choose getNum busses
    let result = interpretBusses arrival justNums
    let indexedBusses = List.indexed busses
    printfn "Earliest bus * minutes to wait: %i" (result * (result - arrival % result))

    let numIndices =
        List.filter (fun (_, x) -> getNum x <> None) indexedBusses
        |> List.map (fst >> int64)

    // Since numbers are co-prime, results list will repeat with a period of N and is symmetrical.
    let result2 =
        List.reduce (*) justNums - crt numIndices justNums

    printfn "Optimal competition timestamp: %i" result2
