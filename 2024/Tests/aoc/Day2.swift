import XCTest

@testable import Day2

class Day2Tests: XCTestCase {
    func testDay2() throws {
        let output = main(
            """
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            """)

        XCTAssertEqual(output.0, 2)
        XCTAssertEqual(output.1, 4)
    }
}
