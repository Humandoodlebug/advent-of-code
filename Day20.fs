module AdventOfCode2020.Day20

open System.IO

type Tile =
    { id: int64
      top: char list
      bottom: char list
      left: char list
      right: char list
      data: char list list }

let input () =
    let lines = File.ReadAllLines "input/day20.txt" |> Array.toList

    let readTile (lines: string list) =
        if List.length lines = 0 then
            None
        else
            let [ idLine ], lines' = List.splitAt 1 lines
            let tileLines, lines'' = List.splitAt 10 lines'

            Some
                ({ id = int64 <| idLine.Substring(5, 4)
                   top = tileLines.Head.ToCharArray() |> Array.toList
                   bottom = Array.rev <| (List.last tileLines).ToCharArray() |> Array.toList
                   left = List.rev <| List.map (fun (x: string) -> x.[0]) tileLines
                   right = List.map (fun (x: string) -> x.[x.Length - 1]) tileLines
                   data = List.map (fun (x: string) -> x.ToCharArray() |> Array.toList) tileLines },
                 if lines''.Length > 0 then List.skip 1 lines'' else lines'')

    List.unfold readTile lines

let findCorners (tiles: Tile list) =
    let tops = List.map (fun t -> t.top) tiles
    let bottoms = List.map (fun t -> t.bottom) tiles
    let lefts = List.map (fun t -> t.left) tiles
    let rights = List.map (fun t -> t.right) tiles
    let allSides = tops @ bottoms @ lefts @ rights
    let counts = List.countBy id allSides |> Map.ofList
    let edgeCount t = List.sumBy (fun x -> if counts.[x] + (if counts.ContainsKey(List.rev x) then counts.[List.rev x] else 0) = 1 then 1 else 0) [ t.top; t.bottom; t.left; t.right ]
    List.filter (edgeCount >> (=) 2) tiles

let rotate (t: Tile) =
    { id = t.id
      top = t.left
      right = t.top
      bottom = t.right
      left = t.bottom
      data = List.transpose t.data |> List.map List.rev }

let flip (t: Tile) =
    { id = t.id
      top = List.rev t.bottom
      bottom = List.rev t.top
      left = List.rev t.left
      right = List.rev t.right
      data = List.rev t.data }

let allAngles t =
    let r1 = rotate t
    let r2 = rotate r1
    let r3 = rotate r2
    let rotations = [ t; r1; r2; r3 ]
    rotations @ List.map flip rotations

let buildImage tiles initialTile =
    let tiles' = List.map (fun f -> f.id, f) tiles |> Map.ofList |> Map.remove initialTile.id
    let find ts left above = Seq.collect (fun t -> Seq.filter (fun t -> t.left = List.rev left.right && t.top = List.rev above.bottom) (allAngles t)) ts
    let findFirst ts above = Seq.collect (fun t -> Seq.filter (fun t -> t.top = List.rev above.bottom) (allAngles t)) ts
    let findFirstLine ts left = Seq.collect (fun t -> Seq.filter (fun t -> t.left = List.rev left.right) (allAngles t)) ts
    let justTiles = Map.toSeq >> Seq.map snd
    let calcNext (ts, img) = seq {
        if List.length img = 1 then
            let results = findFirstLine (justTiles ts) (List.last img.Head)
            for r in results do
                yield Map.remove r.id ts, [List.append img.[0] [r]]
            let results' = findFirst (justTiles ts) (img.Head.Head)
            for r in results' do
                yield Map.remove r.id ts, List.append img [[r]]
        elif (List.last img).Length < img.Head.Length then
            let left = List.last img |> List.last
            let above = img.[img.Length - 2].[List.length (List.last img)]
            let results = find (justTiles ts) left above
            for r in results do
                yield Map.remove r.id ts, List.append (img.[..img.Length - 2]) [List.append <| List.last img <| [r]]
        else
            let results = findFirst (justTiles ts) (List.last img).Head
            for r in results do
                yield Map.remove r.id ts, List.append img [[r]]
    }
    let calcImage initialTile =
        let mutable possibleResults = seq { tiles', [[initialTile]]}
        for i = 1 to tiles.Length - 1 do
            possibleResults <- Seq.collect calcNext possibleResults
        Seq.map snd possibleResults

    Seq.collect calcImage (allAngles initialTile) |> Seq.toList

let printImg (img: Tile list list) =
    for i = 0 to List.length img - 1 do
        for l = 1 to img.[0].[0].data.Length - 2 do
            for j = 0 to img.[i].Length - 1 do
                for k = 1 to img.[i].[j].data.[l].Length - 2 do
                    printf "%c" <| img.[i].[j].data.[l].[k]
            printfn ""

let pieceTogetherImage (img: Tile list list) =
    [|
        for i = 0 to List.length img - 1 do
            for l = 1 to img.[0].[0].data.Length - 2 do
                yield [|
                    for j = 0 to img.[i].Length - 1 do
                        for k = 1 to img.[i].[j].data.[l].Length - 2 do
                            yield img.[i].[j].data.[l].[k]
                |]
    |]

let printTileNums (img: Tile list list) =
    for l in img do
        for t in l do
            printf " %i " t.id
        printfn ""

let getTileNums (img: Tile list list) = List.map (List.map (fun x -> x.id)) img

let seaMonsterPositions = [1, 0; 2, 1; 2, 4; 1, 5; 1, 6; 2, 7; 2, 10; 1, 11; 1, 12; 2, 13; 2, 16; 1, 17; 1, 18; 0, 18; 1, 19]

let findSeaMonsters (image: char [] []) = [
    for i = 0 to image.Length - 3 do
        for j = 0 to image.[0].Length - 20 do
            if List.forall (fun (x, y) -> image.[x + i].[y + j] = '#') seaMonsterPositions then
                yield i, j
]

let run () =
    let tiles = input ()
    let corners = findCorners tiles
    printfn "Corners: %A" (List.map (fun x -> x.id) corners)
    let answer = List.map (fun t -> t.id) corners |> List.reduce (*)
    printfn "Part 1 answer: %i" answer
    let img = buildImage tiles corners.[1]
    printfn "img length: %i" <| List.length img
    printfn "img lengths: %A" <| List.map (List.length) img.Head
    let image = pieceTogetherImage img.[1]
    let hashCount = Array.collect (Array.filter (fun c -> c = '#')) image |> Array.length
    let seaMonsters = findSeaMonsters image
    printfn "Sea monster positions: %A" seaMonsters
    printfn "Water roughness: %i" <| hashCount - (List.length seaMonsters * 15)
