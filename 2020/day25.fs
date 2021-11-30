module Day25

open System.IO

let input () =
    let lines = File.ReadAllLines("input/day25.txt")
    let cardPubKey = int64(lines.[0])
    let doorPubKey = int64(lines.[1])
    cardPubKey, doorPubKey

let guessLoopSize pubKey =
    let mutable value = 1L
    let mutable i = 0
    while value <> pubKey do
        value <- (value * 7L) % 20201227L
        i <- i + 1
    i

let transform loopSize subject =
    let mutable value = 1L
    for i in 1..loopSize do
        value <- (value * subject) % 20201227L
    value

let run () =
    let cardPubKey, doorPubKey = input ()
    let cardLoopSize = guessLoopSize cardPubKey
    let secretKey = transform cardLoopSize doorPubKey
    printfn "Secret key: %i" secretKey
