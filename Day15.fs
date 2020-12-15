module AdventOfCode2020.Day15

let input = [ 10; 16; 6; 0; 1; 17 ]

let game (s :: ss) =
    seq {
        yield s
        let mutable mem = Map.ofList [ (s, 1L) ]
        let mutable prev = s
        let mutable turn = 1L

        for i in ss do
            yield i
            mem <- mem.Add(prev, turn)
            prev <- i
            turn <- turn + 1L

        while true do
            if mem.ContainsKey prev then
                let x = turn - mem.[prev]
                yield x
                mem <- mem.Add(prev, turn)
                prev <- x
            else
                yield 0L
                mem <- mem.Add(prev, turn)
                prev <- 0L

            turn <- turn + 1L
    }

let run () =
    let moves = game [ 10L; 16L; 6L; 0L; 1L; 17L ]
    let answer1 = Seq.item 2019 moves

    printfn "2020th number spoken: %i" answer1

    let answer2 = Seq.item 29_999_999 moves

    printfn "30,000,000th number spoken: %i" answer2
