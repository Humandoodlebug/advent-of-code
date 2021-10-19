module AdventOfCode2020.Day16

open System
open System.IO
open System.Text.RegularExpressions

type Field =
    { name: String
      min1: int64
      max1: int64
      min2: int64
      max2: int64 }

let input () =
    let lines = File.ReadAllLines "input/day16.txt"

    let fieldConstraintLines =
        Array.takeWhile (not << String.IsNullOrWhiteSpace) lines

    let myTicketLine = lines.[fieldConstraintLines.Length + 2]

    let nearbyTicketLines =
        Array.skip (fieldConstraintLines.Length + 5) lines

    let constraintRegex =
        Regex("^(.+): (\d+)-(\d+) or (\d+)-(\d+)$")

    let parseFieldConstraints s =
        let m = constraintRegex.Match s

        if not m.Success then
            failwithf "Failed to parse constraint: %s" s
        else
            { name = m.Groups.[1].Value
              min1 = int64 m.Groups.[2].Value
              max1 = int64 m.Groups.[3].Value
              min2 = int64 m.Groups.[4].Value
              max2 = int64 m.Groups.[5].Value }

    let fields =
        Array.map parseFieldConstraints fieldConstraintLines
        |> Array.toList

    let myTicket =
        myTicketLine.Split(',')
        |> Array.map int64
        |> Array.toList

    let nearbyTickets =
        Array.map (fun (l: String) -> Array.map int64 <| l.Split(',') |> Array.toList) nearbyTicketLines
        |> Array.toList

    fields, myTicket, nearbyTickets

let checkValid field value =
    value >= field.min1 && value <= field.max1
    || value >= field.min2 && value <= field.max2

let checkValidAny fields value =
    Seq.exists (fun f -> checkValid f value) fields

let checkTicketValid fields ticket = Seq.forall (checkValidAny fields) ticket

let getInvalidValues fields ticket =
    List.filter (not << checkValidAny fields) ticket

let getPossibleFields fields value =
    List.filter (fun f -> checkValid f value) fields

let calculateField fields values =
    List.fold getPossibleFields fields values

let run () =
    let fields, myTicket, nearbyTickets = input ()

    let result =
        List.collect (getInvalidValues fields) nearbyTickets
        |> List.reduce (+)

    printfn "Number of invalid tickets: %i" result

    let validTickets =
        List.filter (checkTicketValid fields) nearbyTickets

    let valuesLists = List.transpose validTickets

    let possibleFieldsLists =
        List.map (calculateField fields) valuesLists

    let mutable set = Set.empty

    let eliminate possibleFields =
        if List.length possibleFields = 1 then
            set <- set.Add(List.exactlyOne possibleFields)
            possibleFields
        else
            List.filter (not << set.Contains) possibleFields

    let mutable tFieldOrder = possibleFieldsLists
    for i = 1 to validTickets.Length do
        tFieldOrder <- List.map eliminate tFieldOrder

    let fieldOrder = List.map List.exactlyOne tFieldOrder
    let myTicketPaired = List.zip fieldOrder myTicket

    let departureFields =
        List.filter (fun (f, _) -> f.name.StartsWith("departure")) myTicketPaired

    let result2 =
        List.fold (fun t (_, x) -> t * int64 x) 1L departureFields

    printfn "Departure field multiplication: %i" result2
