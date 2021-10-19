module Day22

open System.IO

let input () =
    let lines = List.ofArray <| File.ReadAllLines("input/day22.txt")
    let deck1 = List.skip 1 lines |> List.takeWhile ((<>) "") |> List.map int64
    let deck2 = List.skipWhile ((<>) "") lines |> List.skip 2 |> List.map int64
    deck1, deck2

let play1 (c1::deck1, c2::deck2) =
    if c1 > c2 then
        deck1 @ [c1; c2], deck2
    else
        deck1, deck2 @ [c2; c1]

let rec play =
    function
    | _::_, _::_ as decks -> play <| play1 decks
    | decks -> decks

let rec subPlay prev =
    function
    | c1::d1 as deck1, c2::d2 as decks ->
        if Set.contains decks prev then
            deck1, []
        elif d1.Length < int c1 || d2.Length < int c2 then
            subPlay (Set.add decks prev) (play1 decks)
        else
            let d1', _ = subPlay Set.empty (List.take (int c1) d1, List.take (int c2) d2)
            subPlay (Set.add decks prev) (if List.isEmpty d1' then d1, d2 @ [ c2; c1 ] else d1 @ [ c1; c2 ], d2)
    | decks -> decks

let run () =
    let decks = input ()

    let deck1, deck2 = play decks
    let winningDeck = if List.isEmpty deck1 then deck2 else deck1
    let winningScore = List.sum <| List.map2 (*) winningDeck [ int64 winningDeck.Length .. -1L .. 1L ]
    printfn "Winning score: %i" winningScore
    let rDeck1, rDeck2 = subPlay Set.empty decks
    let rWinningDeck = if List.isEmpty rDeck1 then rDeck2 else rDeck1
    let rWinningScore = List.sum <| List.map2 (*) rWinningDeck [ int64 rWinningDeck.Length .. -1L .. 1L ]
    printfn "Recursive winning score: %i" rWinningScore
