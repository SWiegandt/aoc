import XCTest

@testable import Day1

class Day1Tests: XCTestCase {
    func testDay1() {
        let output = main(
            """
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            """)

        XCTAssertEqual(output.0, 11)
        XCTAssertEqual(output.1, 31)
    }
}
