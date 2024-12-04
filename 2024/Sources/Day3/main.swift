import Util

func one(_ input: String) -> Int {
    input.matches(of: /mul\((\d+),(\d+)\)/).map {
        Int($0.1)! * Int($0.2)!
    }.reduce(0, +)
}

func two(_ input: String) -> Int {
    input.split(separator: "do()").map {
        one(String($0.split(separator: "don't()")[0]))
    }.reduce(0, +)
}

public func main(_ input: String) -> (Int, Int) {
    (one(input), two(input))
}

try print(main(readInput(3)))
