module AdventOfCode2020.Day15

open System.Collections.Generic

let input = [ 10L; 16L; 6L; 0L; 1L; 17L ]

let game (s :: ss) =
    seq {
        yield s
        let mem = Dictionary<int64, int64>()
        mem.[s] <- 1L
        let mutable prev = s
        let mutable turn = 1L

        for i in ss do
            yield i
            mem.[prev] <- turn
            prev <- i
            turn <- turn + 1L

        while true do
            if mem.ContainsKey prev then
                let x = turn - mem.[prev]
                yield x
                mem.[prev] <- turn
                prev <- x
            else
                yield 0L
                mem.[prev] <- turn
                prev <- 0L

            turn <- turn + 1L
    }

let run () =
    let moves = game input
    let answer1 = Seq.item 2019 moves
    printfn "2020th number spoken: %i" answer1
    let answer2 = Seq.item 29_999_999 moves
    printfn "30,000,000th number spoken: %i" answer2
