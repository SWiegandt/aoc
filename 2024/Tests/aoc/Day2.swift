import Testing

@testable import Day2

@Test
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

    #expect(output == (2, 4))
}
