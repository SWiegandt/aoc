import XCTest

@testable import Day3

class Day3Tests: XCTestCase {
    func testDay3() {
        let outputOne = main(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        ).0

        let outputTwo = main(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        ).1

        XCTAssertEqual(outputOne, 161)
        XCTAssertEqual(outputTwo, 48)
    }
}
