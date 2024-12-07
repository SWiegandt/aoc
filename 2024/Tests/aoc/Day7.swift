import XCTest

@testable import Day7

class Day7Tests: XCTestCase {
    func testDay7() {
        let output = main(
            """
            190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20
            """
        )

        XCTAssertEqual(output.0, 3749)
        XCTAssertEqual(output.1, 11387)
    }
}
