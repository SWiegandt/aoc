(ns test
  (:require
   [clojure.test :as t]
   [day1]
   [day2]
   [day3]))

(t/testing "Day 1"
  (let [input "1721
979
366
299
675
1456"]
    (t/is (= (day1/p1 input) 514579))
    (t/is (= (day1/p2 input) 241861950))))

(t/testing "Day 2"
  (let [input "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"]
    (t/is (= (day2/p1 input) 2))
    (t/is (= (day2/p2 input) 1))))

(t/testing "Day 3"
  (let [input "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"]
    (t/is (= (day3/p1 input) 7))
    (t/is (= (day3/p2 input) 336))))
