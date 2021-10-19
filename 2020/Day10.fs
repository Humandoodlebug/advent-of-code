module AdventOfCode2020.Day10

let input () =
    Util.readFileInts "input/day10.txt"
    |> Seq.map (fun [| x |] -> x)
    |> Seq.toList

let countWays diffs =
    let inc (lst, ones) =
        function
        | 1L -> lst, ones + 1L
        | 3L -> if ones < 2L then lst, 0L else ones :: lst, 0L
        | x -> failwithf "Unrecognised diff '%i'" x

    let counts, _ = List.fold inc ([], 0L) diffs

    let rec countWays' x =
        if x = 0L then
            1L
        elif x < 0L then
            0L
        else
            countWays' (x - 1L)
            + countWays' (x - 2L)
            + countWays' (x - 3L)

    List.map countWays' counts |> List.fold (*) 1L

let run () =
    let inp = input ()
    let sorted = 0L :: List.sort inp

    let diffs =
        List.zip sorted (sorted.Tail @ [ List.last sorted + 3L ])
        |> List.map (fun (a, b) -> b - a)

    let diffMap = List.groupBy id diffs |> Map.ofList
    let result = diffMap.[1L].Length * diffMap.[3L].Length
    printfn "Jolt multiplication result: %i" result

    let wayCount = countWays diffs

    printfn "Ways of charging your phone: %i" wayCount
