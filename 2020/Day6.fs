module AdventOfCode2020.Day6

open System
open System.IO

let input () =
    File.ReadAllText("input/day6.txt").Replace("\r", String.Empty).Replace('\n', ' ').TrimEnd()

let groups () =
    input().Split("  ")
    |> Array.map (fun x ->
        x.Split(' ')
        |> Array.map (fun x -> x.ToCharArray()))

let run () =
    let gs = groups ()

    let uniques =
        Array.map (fun g -> Array.concat g |> Array.distinct) gs

    let counts = Array.map Array.length uniques
    let sum = Array.sum counts
    printfn "Sum of questions anyone answered yes to: %i" sum
    let sets = Array.map (Array.map Set.ofArray) gs
    let shared = Array.map Set.intersectMany sets
    let counts = Array.map Set.count shared
    let sum = Array.sum counts
    printfn "Sum of questions everyone answered yes to: %i" sum
