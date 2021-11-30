module Day23

open System
open System.IO
open System.Linq

let input () =
    let line = File.ReadAllLines("input/day23.txt").Single()
    let cupsChars = List.ofSeq line
    List.map (fun x -> int(x.ToString())) cupsChars

let move (current::c1::c2::c3::rest) =
    let mutable dest = current - 1
    while not (rest.Contains dest) do
        dest <- dest - 1
        if dest <= 0 then
            dest <- rest.Max()
    let _ :: afterDest = List.skipWhile (fun x -> x <> dest) rest
    let newRest = List.takeWhile (fun x -> x <> dest) rest |> List.append <| dest :: c1 :: c2 :: c3 :: afterDest
    newRest @ [current]

let parseToMap (cups: int list) =
    let nexts = cups.Tail @ [cups.Head]
    let mutable map = Map.ofList <| List.map2 (fun c n -> c,n) cups nexts
    map <- map.Add(cups.Last(), 10)
    for i in 11..1_000_000 do
        map <- map.Add(i-1, i)
    map <- map.Add(1_000_000, cups.Head)
    map

let moveMap (map: Map<int, int>) (current: int) : Map<int, int> * int =
    let mutable map = map

    let c1 = map.[current]
    let c2 = map.[c1]
    let c3 = map.[c2]
    let next = map.[c3]

    let mutable dest = if current = 1 then map.Count else current - 1
    while ([c1; c2; c3].Contains(dest)) do
        dest <- dest - 1
        if dest <= 0 then
            dest <- map.Count

    let afterDest = map.[dest]

    map <- map.Add(current, next)
    map <- map.Add(dest, c1)
    map <- map.Add(c3, afterDest)
    map, map.[current]

let run () =
    let inp = input ()
    let mutable cups = inp
    for i in 1..100 do
        cups <- move cups
    let _::ordered = List.skipWhile (fun x -> x <> 1) cups @ List.takeWhile (fun x -> x <> 1) cups
    printfn "Final State (pt. 1): %s" <| String.Join("", ordered)
    let mutable map = parseToMap inp
    let mutable current = inp.Head
    for i in 1..10_000_000 do
        let m, c = moveMap map current
        map <- m
        current <- c
    let ans1 = map.[1]
    let ans2 = map.[ans1]
    printfn "Answer (pt. 2): %i" (int64(ans1) * int64(ans2))
