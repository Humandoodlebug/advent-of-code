module AdventOfCode2020.Day2

open System.IO

let input () = File.ReadAllLines("input/day2.txt")

let split () =
    [ for l in input () -> let [| rule; password |] = l.Split(": ") in rule, password ]

let processed () =
    [ for (rule, password) in split () ->
        let [| range; letter |] = rule.Split(' ')
        let [| min; max |] = Array.map int (range.Split('-'))
        ((min, max), letter.[0], password.ToCharArray()) ]

let validPasswords () =
    List.filter (fun ((min, max), letter, password) ->
        let count =
            Array.filter (fun x -> x = letter) password
            |> Array.length

        min <= count && count <= max) (processed ())

let validPasswords2 () =
    List.filter (fun ((min, max), letter, password: char []) ->
        let pos1, pos2 = min - 1, max - 1
        let c1 = password.[pos1] = letter
        let c2 = password.[pos2] = letter
        (c1 || c2) && not (c1 && c2)) (processed ())


let run () =
    printfn "Valid passwords: %i"
    <| validPasswords().Length
    printfn "Valid passwords 2: %i"
    <| validPasswords2().Length
