module AdventOfCode2020.Day17

open System.IO

let input () =
    let lines = File.ReadAllLines("input/day17.txt")

    Array.fold (fun s (x, l: string) ->
        Array.indexed (l.ToCharArray())
        |> Array.fold (fun s (y, c) -> if c = '#' then Set.add [ x; y; 0 ] s else s) s) Set.empty (Array.indexed lines)

let around coords =
    let rec around' (coords: int list) =
        match coords with
        | [ c ] -> [ c - 1 .. c + 1 ] |> List.map (fun x -> [ x ])
        | c :: cs ->
            let cL = [ c - 1 .. c + 1 ]
            List.collect (fun cs -> List.map (fun c -> c :: cs) cL) (around' cs)

    around' coords |> List.filter ((<>) coords)

let simulate (poss: 'a Set) =
    let inc posC pos =
        if Map.containsKey pos posC then Map.add pos (posC.[pos] + 1) posC else Map.add pos 1 posC

    let posCounts =
        Set.fold (fun counts p -> Seq.fold inc counts <| around p) Map.empty poss

    Map.toSeq posCounts
    |> Seq.filter (fun (p, c) -> c = 3 || c = 2 && Set.contains p poss)
    |> Seq.map fst
    |> Set.ofSeq

let rec runMany x f =
    match x with
    | 0 -> id
    | n when n > 0 -> f >> runMany (x - 1) f
    | n -> failwith $"Expected {nameof x} to be at least 0; Got '{n}'."


let append0Index xs = xs @ [ 0 ]

let run () =
    let inp = input ()
    let finalState = runMany 6 simulate inp
    let answer = Set.count finalState
    printfn "Number of active cubes after 6 cycles: %i" answer
    let inp4D = Set.map append0Index inp
    let finalState4D = runMany 6 simulate inp4D
    let answer4D = Set.count finalState4D
    printfn "Number of active cubes after 6 cycles in 4D: %i" answer4D
