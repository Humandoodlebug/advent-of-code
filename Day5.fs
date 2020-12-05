module AdventOfCode2020.Day5

open System.IO

let input () =
    File.ReadAllLines "input/day5.txt" |> Array.toList

let seatId (s: string) =
    let cToI =
        function
        | 'F' -> 0
        | 'B' -> 1
        | 'L' -> 0
        | 'R' -> 1

    let ints = List.map cToI <| List.ofSeq s
    List.fold (fun acc x -> (acc <<< 1) + x) 0 ints

let run () =
    let seatIds = List.map seatId <| input ()
    let maxSeatId = List.max seatIds
    printfn "Max seat ID: %i" maxSeatId
    let minSeatId = List.min seatIds
    let allIds = [ minSeatId .. maxSeatId ]
    let [ mySeat ] = List.except seatIds allIds
    printfn "Your seat: %i" mySeat
