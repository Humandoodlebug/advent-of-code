module AdventOfCode2020.RunAll

let runDay day run =
    printfn "====== Day %i ======" day
    Util.time run
    printfn ""

[<EntryPoint>]
let main argv =
    runDay 1 Day1.run
    runDay 2 Day2.run
    runDay 3 Day3.run
    runDay 4 Day4.run
    runDay 5 Day5.run
    runDay 6 Day6.run
    runDay 7 Day7.run
    runDay 8 Day8.run
    runDay 9 Day9.run
    runDay 10 Day10.run
    runDay 11 Day11.run
    runDay 12 Day12.run
    runDay 13 Day13.run
    runDay 14 Day14.run
    runDay 15 Day15.run
    runDay 16 Day16.run
    0
