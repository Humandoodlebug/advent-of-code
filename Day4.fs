module AdventOfCode2020.Day4

open System
open System.IO


let input () =
    File.ReadAllLines "input/day4.txt" |> Array.toList

let parseInput inp =
    let parseField (f: string) =
        let [| n; v |] = f.Split(':')
        n, v

    let parseLine =
        function
        | "" -> []
        | l -> Array.map parseField (l.Split ' ') |> Array.toList

    let folder (maps: Map<string, string> list) fields =
        match maps with
        | [] ->
            match fields with
            | [] -> []
            | ls -> [ List.fold (fun m (n, v) -> Map.add n v m) Map.empty ls ]
        | m :: ms ->
            match fields with
            | [] -> if m = Map.empty then m :: ms else Map.empty :: m :: ms
            | ls ->
                List.fold (fun m (n, v) -> Map.add n v m) m ls
                :: ms

    List.fold folder [] <| List.map parseLine inp

let trimEnd (s: string) i = s.Substring(0, s.Length - i)

let validators =
    [ "byr", (fun s -> let x = int s in x >= 1920 && x <= 2002)
      "iyr", (fun s -> let x = int s in x >= 2010 && x <= 2020)
      "eyr", (fun s -> let x = int s in x >= 2020 && x <= 2030)
      "hgt",
      (fun (s: string) ->
          s.EndsWith("cm")
          && (let x = int (trimEnd s 2) in x >= 150 && x <= 193)
          || s.EndsWith("in")
             && (let x = int (trimEnd s 2) in x >= 59 && x <= 76))
      "hcl",
      (fun (s: string) ->
          s.StartsWith('#')
          && Array.forall (fun x -> List.contains x ([ '0' .. '9' ] @ [ 'a' .. 'f' ])) (s.Substring(1).ToCharArray()))
      "ecl", fun (s: string) -> List.contains s ["amb"; "blu"; "brn"; "gry"; "grn"; "hzl"; "oth"]
      "pid", fun (s: string) -> s.Length = 9 && Array.forall (fun x -> List.contains x [ '0' .. '9' ]) (s.ToCharArray())
    ]

let validateField (m: Map<string, string>) (k, f) =
    match m.TryFind k with
    | Some s -> f s
    | None -> failwith "Couldn't find required field in map."

let run () =
    let maps = parseInput <| input ()

    let keys = List.map fst validators

    let validMaps =
        List.filter (fun (m: Map<string, string>) -> List.forall (fun k -> m.ContainsKey k) keys) maps

    let validCount = List.length validMaps
    printfn "Passports with required fields: %i" validCount
    let validatedMaps = List.filter (fun (m: Map<string, string>) -> List.forall (fun v -> validateField m v) validators) validMaps
    let validatedCount = List.length validatedMaps
    printfn "Passports with valid required fields: %i" validatedCount
