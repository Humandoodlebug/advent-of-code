module AdventOfCode2020.Day8

open System.IO

type Inst =
    | Acc of int
    | Jmp of int
    | Nop of int

let strToInst =
    function
    | "acc" -> Acc
    | "jmp" -> Jmp
    | "nop" -> Nop
    | str -> failwithf "'%s' is not a recognised operation." str

let input () =
    let processLine (l: string) =
        let [| inst; x |] = l.Split ' '
        (inst, int x)

    File.ReadAllLines "input/day8.txt"
    |> Array.map processLine
    |> Array.map (fun (inst, x) -> strToInst inst x)

let rec exec (insts: Inst []) (prev: int Set) (acc: int) (pos: int) =
    if Set.contains pos prev || pos > insts.Length then
        acc, false
    else if pos = insts.Length then
        acc, true
    else
        match insts.[pos] with
        | Acc x -> exec insts (Set.add pos prev) (acc + x) (pos + 1)
        | Jmp x -> exec insts (Set.add pos prev) acc (pos + x)
        | Nop _ -> exec insts (Set.add pos prev) acc (pos + 1)

let swapInsts =
    function
    | Jmp x -> Nop x
    | Nop x -> Jmp x
    | Acc x -> Acc x

let run () =
    let inp = input ()
    let acc, _ = exec inp Set.empty 0 0
    printfn "Accumulator value before loop: %i" acc
    for i = 0 to inp.Length - 1 do
        inp.[i] <- swapInsts inp.[i]
        let acc, b = exec inp Set.empty 0 0
        if b
        then printfn "Accumulator value after fix: %i" acc
        inp.[i] <- swapInsts inp.[i]
