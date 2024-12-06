import XCTest

@testable import Day4

class Day4Tests: XCTestCase {
    func testDay4() {
        let output = main(
            """
            MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX
            """
        )

        XCTAssertEqual(output.0, 18)
        XCTAssertEqual(output.1, 9)
    }
}
