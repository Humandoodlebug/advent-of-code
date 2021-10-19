module AdventOfCode2020.Day9

open System.Collections.Generic

let input =
    Util.readFileInts "input/day9.txt"
    |> Seq.map (fun [| x |] -> x)

let isSumOf (x: int64) (list: int64 LinkedList) (set: int64 Set) =
    Seq.exists (fun e -> (x - e) <> e && set.Contains(x - e)) list

let iterate (inp: int64 list) =
    let list = LinkedList<int64>()
    let mutable set = Set.empty
    let mutable rest = inp
    while list.Count < 25 do
        let x :: xs = rest
        rest <- xs
        list.AddFirst x |> ignore
        set <- Set.add x set
    let mutable result = None
    while result = None do
        let x :: xs = rest
        rest <- xs
        if not <| isSumOf x list set then
            result <- Some x
        else
            list.AddFirst x |> ignore
            Set.remove list.Last.Value set |> ignore
            list.RemoveLast()
            set <- Set.add x set
    let (Some r) = result
    r

let window size (list: int64 seq) =
    seq {
        let buffer = LinkedList<int64>()
        let mutable sum = 0L
        for x in list do
            if buffer.Count < size then
                buffer.AddFirst x |> ignore
                sum <- sum + x
            else
                yield (Seq.min buffer + Seq.max buffer, sum)
                buffer.AddFirst x |> ignore
                sum <- sum + x - buffer.Last.Value
                buffer.RemoveLast()
        if buffer.Count = size then yield (Seq.min buffer + Seq.max buffer, sum)
    }

let findSumTo x inp =
    let rec findSumTo' x inp size =
        let ws = window size inp
        let result = Seq.tryFind (fun (_, sum) -> x = sum) ws
        match result with
        | None -> findSumTo' x inp (size + 1)
        | Some (v, _) -> v

    findSumTo' x inp 2


let run () =
    let inp = Seq.toList input
    let result = iterate inp
    printfn "Not the sum of pair from previous 25: %i" result
    let weakness = findSumTo result inp
    printfn "Weakness: %i" weakness
