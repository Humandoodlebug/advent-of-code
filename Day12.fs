module AdventOfCode2020.Day12

open System.IO

type Direction =
    | DNorth
    | DEast
    | DSouth
    | DWest

type Instruction =
    | North of int
    | East of int
    | South of int
    | West of int
    | Left of int
    | Right of int
    | Forward of int

type Position =
    { north: int
      east: int
      direction: int }

type WPosition =
    { north: int
      east: int
      wNorth: int
      wEast: int }

let input () =
    let raw =
        File.ReadAllLines "input/day12.txt"
        |> Array.toList

    let parse (str: string) =
        let i = str.Substring 1 |> int
        match str.[0] with
        | 'N' -> North i
        | 'E' -> East i
        | 'S' -> South i
        | 'W' -> West i
        | 'L' -> Left i
        | 'R' -> Right i
        | 'F' -> Forward i
        | c -> failwithf "Failed to parse instruction character '%c'" c

    List.map parse raw

let handleInstr (pos: Position) instr =
    match instr with
    | North x -> { pos with north = pos.north + x }
    | East x -> { pos with east = pos.east + x }
    | South x -> { pos with north = pos.north - x }
    | West x -> { pos with east = pos.east - x }
    | Right x ->
        { pos with
              direction = (pos.direction + x) % 360 }
    | Left x ->
        { pos with
              direction = (pos.direction + 360 - x) % 360 }
    | Forward x ->
        match pos.direction with
        | 0 -> { pos with north = pos.north + x }
        | 90 -> { pos with east = pos.east + x }
        | 180 -> { pos with north = pos.north - x }
        | 270 -> { pos with east = pos.east - x }
        | i -> failwithf "Unhandled direction '%i'" i

let wHandleInstr (pos: WPosition) instr =
    let rotate90 pos =
        { pos with
              wEast = pos.wNorth
              wNorth = -pos.wEast }

    let rec rotate =
        function
        | 0 -> id
        | 90 -> rotate90
        | 180 -> rotate90 >> rotate90
        | 270 -> rotate90 >> rotate90 >> rotate90
        | x ->
            if x >= 360 then rotate (x % 360)
            elif x < 0 then rotate (360 + x % 360)
            else failwithf "Unhandled direction '%i'" x

    match instr with
    | North x -> { pos with wNorth = pos.wNorth + x }
    | East  x -> { pos with wEast = pos.wEast + x }
    | South x -> { pos with wNorth = pos.wNorth - x }
    | West  x -> { pos with wEast = pos.wEast - x }
    | Right x -> rotate x pos
    | Left  x -> rotate -x pos
    | Forward x ->
        { pos with
              north = pos.north + x * pos.wNorth
              east = pos.east + x * pos.wEast }

let manhattan x y = abs x + abs y

let run () =
    let inp = input ()

    let finalPosition =
        List.fold handleInstr { north = 0; east = 0; direction = 90 } inp

    let answer =
        manhattan finalPosition.north finalPosition.east

    printfn "Manhattan distance: %i" answer

    let wFinalPosition =
        List.fold
            wHandleInstr
            { north = 0
              east = 0
              wNorth = 1
              wEast = 10 }
            inp

    let wAnswer =
        manhattan wFinalPosition.north wFinalPosition.east

    printfn "Manhattan distance with waypoint: %i" wAnswer
