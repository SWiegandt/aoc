import XCTest

@testable import Day6

class Day6Tests: XCTestCase {
    func testDay6() {
        let output = main(
            """
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            """
        )

        XCTAssertEqual(output.0, 41)
        XCTAssertEqual(output.1, 6)
    }
}
