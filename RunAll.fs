module AdventOfCode2020.RunAll

let runDay day run =
    printfn "====== Day %i ======" day
    run ()
    printfn ""

[<EntryPoint>]
let main argv =
    runDay 1 Day1.run
    runDay 2 Day2.run
    0
