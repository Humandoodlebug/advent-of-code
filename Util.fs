module AdventOfCode2020.Util

open System
open System.IO

let ParseLineInts (line: String): int64 [] = line.Split ' ' |> Array.map int64

let readAllInts (reader: TextReader) =
    seq {
        let mutable line = reader.ReadLine()
        while line <> null do
            yield ParseLineInts line
            line <- reader.ReadLine()
    }

let readInInts () = readAllInts Console.In

let readFileInts path =
    seq {
        use reader = File.OpenText path
        yield! readAllInts reader
    }
