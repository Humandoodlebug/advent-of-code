module Day24

open System.IO

type Tile = (int * int)

let add (a1, a2) (b1, b2) = (a1+b1, a2+b2)

let parseTile (s: string) =
    let rec parse =
        function
        | [] -> (0,0)
        | 'e'::rs -> add (0,2) <| parse rs
        | 's'::'e'::rs -> add (-1,1) <| parse rs
        | 's'::'w'::rs -> add (-1,-1) <| parse rs
        | 'w'::rs -> add (0,-2) <| parse rs
        | 'n'::'w'::rs -> add (1,-1) <| parse rs
        | 'n'::'e'::rs -> add (1,1) <| parse rs
    parse <| Array.toList (s.ToCharArray())

let input () =
    let lines = File.ReadAllLines("input/day24.txt")
    Seq.toList <| Seq.map parseTile lines

let genAdjacent p = List.map (add p) [(0,2); (-1,1); (-1,-1); (0,-2); (1,-1); (1,1)]


let turnBlack (blacks: Tile Set) tile =
    let adjacent = genAdjacent tile
    let blackNeighbours = List.filter (blacks.Contains) adjacent
    blackNeighbours.Length = 2

let oneTile (blacks: Tile Set) (tile: Tile) =
    let adjacent = genAdjacent tile
    let blackNeighbours = List.filter (blacks.Contains) adjacent
    let whiteNeighbours = List.except blackNeighbours adjacent
    let newBlacks = List.filter <| turnBlack blacks <| whiteNeighbours
    if blackNeighbours.Length > 0 && blackNeighbours.Length < 3 then
        Set.ofList (tile :: newBlacks)
    else
        Set.ofList newBlacks


let moveDay (blacks: Tile Set) =
    Set.map (oneTile blacks) blacks |> Set.unionMany

let run () =
    let tiles = input ()
    let blackTiles = List.countBy id tiles |> List.filter (fun (_, c) -> c % 2 = 1)
    printfn "Black tile count (pt. 1): %i" blackTiles.Length
    let mutable blacksSet = Set.ofList <| List.map fst blackTiles
    for i in 1..100 do
        blacksSet <- moveDay blacksSet
    printfn "Black tile count (pt. 2): %i" <| blacksSet.Count
