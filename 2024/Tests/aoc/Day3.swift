import Testing

@testable import Day3

@Test
func testDay3() {
    let outputOne = main(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    ).0

    let outputTwo = main(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    ).1

    #expect(outputOne == 161)
    #expect(outputTwo == 48)
}
