module AdventOfCode2020.Day11

open System.IO

let checkBounds arr x y =
    x
    >= 0
    && x < Array2D.length1 arr
    && y >= 0
    && y < Array2D.length2 arr

let adjacent inp i j =
    [ for x in i - 1 .. i + 1 do
        for y in j - 1 .. j + 1 -> x, y ]
    |> List.filter (fun (x, y) -> checkBounds inp x y && not (x = i && y = j))

let inSight inp i j =
    let mods =
        [ for x in -1 .. 1 do
            for y in -1 .. 1 -> x, y ]
        |> List.filter (fun (x, y) -> not (x = 0 && y = 0))

    [ for (xm, ym) in mods do
        let mutable x, y = i + xm, j + ym
        while checkBounds inp x y && inp.[x, y] = '.' do
            x <- x + xm
            y <- y + ym
        if checkBounds inp x y then yield x, y ]

let rec simulate around tolerance inp =

    Array2D.init (Array2D.length1 inp) (Array2D.length2 inp) (fun i j ->
        match inp.[i, j] with
        | '.' -> '.'
        | '#' ->
            if around inp i j
               |> List.filter (fun (x, y) -> inp.[x, y] = '#')
               |> List.length
               >= tolerance then
                'L'
            else
                '#'
        | 'L' ->
            if around inp i j
               |> List.exists (fun (x, y) -> inp.[x, y] = '#') then
                'L'
            else
                '#')

let rec stabilise around tolerance inp =

    let isEqual (arr1: 'a [,]) (arr2: 'a [,]) =
        Array2D.length1 arr1 = Array2D.length1 arr2
        && Array2D.length2 arr1 = Array2D.length2 arr2
        && seq {
            for i = 0 to Array2D.length1 arr1 - 1 do
                for j = 0 to Array2D.length2 arr1 - 1 do
                    yield arr1.[i, j], arr2.[i, j]
           }
           |> Seq.exists (fun (x, y) -> x <> y)
           |> not

    let arr = simulate around tolerance inp
    if isEqual inp arr then arr else stabilise around tolerance arr

let array2dToSeq arr =
    seq {
        for i = 0 to Array2D.length1 arr - 1 do
            for j = 0 to Array2D.length2 arr - 1 do
                yield arr.[i, j]
    }

let run () =
    let inp = File.ReadAllLines "input/day11.txt"

    let arr =
        Array2D.init (Array.length inp) (String.length inp.[0]) (fun i j -> inp.[i].[j])

    let result1 =
        stabilise adjacent 4 arr
        |> array2dToSeq
        |> Seq.filter (fun x -> x = '#')
        |> Seq.length

    printfn "Number of seats occupied in scenario 1: %i" result1

    let result2 =
        stabilise inSight 5 arr
        |> array2dToSeq
        |> Seq.filter (fun x -> x = '#')
        |> Seq.length

    printfn "Number of seats occupied in scenario 2: %i" result2
    ()
