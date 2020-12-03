module AdventOfCode2020.Day3

open System.IO

let input () = File.ReadAllLines("input/day3.txt")

let boolMap (input: string []) =
    Array2D.init
    <| input.[0].Length
    <| input.Length
    <| fun x y -> input.[y].[x] = '#'

let countTrees map x' y' =
    let rec countTrees' x y count =
        if y >= Array2D.length2 map
        then count
        else countTrees' ((x + x') % Array2D.length1 map) (y + y') (if map.[x, y] then count + 1 else count)

    countTrees' 0 0 0

let run () =
    let map = boolMap <| input ()
    let slopes = [ (1, 1); (3, 1); (5, 1); (7, 1); (1, 2) ]

    let trees =
        [ for x, y in slopes -> ((x, y), countTrees map x y) ]

    for (x, y), t in trees do
        printfn "Slope right %i, down %i: encountered %i trees." x y t

    let product =
        List.map (fun (_, t) -> t) trees
        |> List.reduce (*)

    printfn "Product: %i" product
