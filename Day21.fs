module AdventOfCode2020.Day21

open System
open System.IO

type Food =
    { ingredients: string Set
      allergens: string Set }
let input () =
    let lines = Array.toList <| File.ReadAllLines("input/day21.txt")
    let parseLine (l: string) =
        let [| ingredientsPart; allergensPart |] = l.Split('(')
        let ingredients = Set.ofArray <| ingredientsPart.Split(' ', StringSplitOptions.RemoveEmptyEntries)
        let allergens = Set.ofArray <| allergensPart.Substring(9).Replace(")", "").Replace(",", "").Split(' ')
        { ingredients = ingredients; allergens = allergens }

    List.map parseLine lines

let getPossibleRelation foods =
    let allIngredients = Set.unionMany <| List.map (fun x -> x.ingredients) foods
    let allAllergens = Set.unionMany <| List.map (fun x -> x.allergens) foods
    let mutable possibleRelation = Map.ofSeq <| seq { for x in allAllergens -> x, allIngredients }

    for f in foods do
        for a in f.allergens do
            possibleRelation <- possibleRelation.Change(a, fun (Some x) -> Some (Set.intersect x f.ingredients))

    possibleRelation

let findNonAllergens foods =
    let allIngredients = Set.unionMany <| List.map (fun x -> x.ingredients) foods
    let possibleRelation = getPossibleRelation foods

    Set.difference allIngredients (Set.unionMany <| Seq.map snd (Map.toSeq possibleRelation))

let getAllergenMapping foods =
    [
        let mutable possibleRelation = getPossibleRelation foods
        while possibleRelation.Count > 0 do
            let principal = (Map.filter (fun _ v -> Set.count v = 1) possibleRelation |> Map.toList).[0]
            let a = fst principal
            let i = (Set.toList (snd principal)).[0]
            yield a, i
            possibleRelation <- Map.remove a possibleRelation
            possibleRelation <- Map.map (fun _ -> Set.remove i) possibleRelation
    ]

let run () =
    let foods = input ()
    let nonAllergens = findNonAllergens foods
    let allergensCount = List.collect (fun n -> List.filter (fun f -> f.ingredients.Contains(n)) foods) <| Set.toList nonAllergens |> List.length
    printfn "Allergen appearance count: %i" allergensCount
    let allergenMapping = getAllergenMapping foods
    let sortedIngredients = List.sortBy fst allergenMapping |> List.map snd
    printfn "Canonical dangerous ingredient list: %s" <| String.Join(',', sortedIngredients)
