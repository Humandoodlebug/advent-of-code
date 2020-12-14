module AdventOfCode2020.Day14

open System.IO
open System.Text.RegularExpressions

type Instr =
    | Mask of char []
    | Mem of int64 * int64

type State<'a> = { memory: Map<int64, int64>; mask: 'a }

let input () =
    let lines = File.ReadAllLines "input/day14.txt"

    let memRegex =
        Regex(@"mem\[(\d+)\] = (\d+)", RegexOptions.Compiled)

    [ for line in lines ->
        if line.StartsWith("mask = ") then
            let mask = line.Remove(0, 7).ToCharArray()
            Mask mask
        else
            let m = memRegex.Match(line)
            if m.Success
            then Mem(int64 m.Groups.[1].Value, int64 m.Groups.[2].Value)
            else failwithf "Couldn't parse instruction: \"%s\"" line ]

let mask (zeros, ones) x = (x &&& zeros) ||| ones

let parseInstr state =
    function
    | Mask m ->
        let zeroParse =
            function
            | '0' -> 0L
            | _ -> 1L

        let oneParse =
            function
            | '1' -> 1L
            | _ -> 0L

        let createMask f arr =
            Seq.fold (fun s x -> (s <<< 1) + f x) 0L arr

        let m0 = createMask zeroParse m
        let m1 = createMask oneParse m

        { state with mask = m0, m1 }
    | Mem (a, v) ->
        let v' = mask state.mask v
        { state with
              memory = if v' = 0L then Map.remove a state.memory else Map.add a v' state.memory }

let genFloatingMasks mask =
    let rec gen (f0, f1) =
        function
        | '0' -> [ (f0 <<< 1) + 1L, (f1 <<< 1) ]
        | '1' -> [ (f0 <<< 1) + 1L, (f1 <<< 1) + 1L ]
        | 'X' ->
            [ (f0 <<< 1), (f1 <<< 1)
              (f0 <<< 1) + 1L, (f1 <<< 1) + 1L ]

    List.fold (fun s c -> List.map (fun x -> gen x c) s |> List.concat) [ (0L, 0L) ] (Array.toList mask)

let parseInstrFloating state =
    function
    | Mask m -> { state with mask = genFloatingMasks m }
    | Mem (a, v) ->
        let a's = List.map (fun x -> mask x a) state.mask
        { state with
              memory = List.fold (fun s a' -> Map.add a' v s) state.memory a's }

let run () =
    let inp = input ()

    let finalState =
        List.fold parseInstr { mask = (0L, 0L); memory = Map.empty } inp

    let answer =
        Map.fold (fun s _ x -> s + x) 0L finalState.memory
    printfn "Memory sum: %i" answer

    let finalStateFloating = List.fold parseInstrFloating { mask = []; memory = Map.empty } inp
    let floatingAnswer = Map.fold (fun s _ x -> s + x) 0L finalStateFloating.memory
    printfn "Floating memory sum: %i" floatingAnswer
