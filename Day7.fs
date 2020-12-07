module AdventOfCode2020.Day7

open System.IO
open System.Text.RegularExpressions

let input () =
    File.ReadAllLines "input/day7.txt" |> List.ofArray

let regex =
    Regex
        (@"(.*) bags contain (?:no other bags|(\d+) ((?:\w|\s)+) bags?(?:, (\d+) ((?:\w|\s)+) bags?)*).",
         RegexOptions.Compiled)

let ruleMap inp =
    let extractMatches l =
        let m = regex.Match(l)
        let c1 = m.Groups.[1].Value

        let c2 =
            (if m.Groups.[3].Success
             then [ m.Groups.[3].Value, int m.Groups.[2].Value ]
             else [])
            @ [ for i = 0 to m.Groups.[5].Captures.Count - 1 do
                    yield m.Groups.[5].Captures.[i].Value, int m.Groups.[4].Captures.[i].Value ]

        c1, Map.ofList c2

    List.map extractMatches inp |> Map.ofList

let reverseMap (rules: Map<string, Map<string, int>>) =
    let revPairs =
        [ for kvp in rules do
            for x in kvp.Value do
                yield kvp.Key, x.Key ]

    let groups = List.groupBy snd revPairs

    List.map (fun (k, xs) -> k, List.map fst xs |> Set.ofList) groups
    |> Map.ofList

let rec iterate revRules (heads: string Set) (searched: string Set) =
    let newSearched = Set.union searched heads

    let newHeads =
        Set.filter (fun x -> Map.containsKey x revRules) heads
        |> Set.map (fun x -> revRules.[x])
        |> Set.unionMany
        |> Set.difference
        <| searched

    if Set.isEmpty newHeads then
        newHeads
    else
        Set.union newHeads
        <| iterate revRules newHeads newSearched

let rec flattenBags (rules: Map<string, Map<string, int>>) bag =
    [ for b in rules.[bag] do
        yield (flattenBags rules b.Key + 1) * b.Value ]
    |> List.sum

let run () =
    let inp = input ()
    let rules = ruleMap inp
    let revRules = reverseMap rules

    let bags =
        iterate revRules (Set.singleton "shiny gold") Set.empty

    printfn "Bag colour count: %i" <| Set.count bags
    printfn "Bags inside shiny gold bag: %i"
    <| flattenBags rules "shiny gold"
