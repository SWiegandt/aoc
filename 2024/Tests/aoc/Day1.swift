import Testing

@testable import Day1

@Test
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

    #expect(output == (11, 31))
}
