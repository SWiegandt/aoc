use std::collections::HashSet;

use regex::Regex;

use aoc2022::util::{read_input, ToInputVec};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Interval(i32, i32);

#[derive(Debug, Clone)]
enum IntervalUnion {
    Empty,
    Single(Interval),
    Union(Vec<Interval>),
}

impl Interval {
    fn new(start: i32, stop: i32) -> Self {
        assert!(start <= stop, "{} <= {}", start, stop);
        Interval(start, stop)
    }

    fn union(self, other: Self) -> IntervalUnion {
        if other.1 < self.0 - 1 || other.0 > self.1 + 1 {
            IntervalUnion::Union(vec![self, other])
        } else {
            IntervalUnion::Single(Interval::new(self.0.min(other.0), self.1.max(other.1)))
        }
    }

    fn length(&self) -> i32 {
        self.1 - self.0 + 1
    }
}

impl IntervalUnion {
    fn add(self, interval: Interval) -> Self {
        match self {
            IntervalUnion::Empty => IntervalUnion::Single(interval),
            IntervalUnion::Single(i) => i.union(interval),
            IntervalUnion::Union(mut is) => {
                for (idx, i) in is.iter().enumerate() {
                    let union = i.union(interval);

                    if let IntervalUnion::Empty = union {
                        unreachable!();
                    } else if let IntervalUnion::Single(i) = union {
                        let mut is_without_merged = vec![];
                        is_without_merged.append(&mut is[0..idx].to_vec());
                        is_without_merged.append(&mut is[idx + 1..].to_vec());

                        return IntervalUnion::Union(is_without_merged).add(i);
                    }
                }

                is.push(interval);

                if let &[i] = &is[..] {
                    IntervalUnion::Single(i)
                } else {
                    IntervalUnion::Union(is)
                }
            }
        }
    }

    fn length(&self) -> i32 {
        match self {
            IntervalUnion::Empty => 0,
            IntervalUnion::Single(i) => i.length(),
            IntervalUnion::Union(is) => is.iter().map(|i| i.length()).sum(),
        }
    }
}

fn find_non_beacon_intervals(input: &Vec<(Pos, Pos)>, target: i32, bounds: Option<(i32, i32)>) -> IntervalUnion {
    let mut intervals = IntervalUnion::Empty;

    for (sensor, beacon) in input {
        let distance_to_beacon = sensor.distance(beacon);
        let distance_to_line = sensor.distance(&Pos(sensor.0, target));
        let interval_length = distance_to_beacon - distance_to_line;
        let mut interval_start: i32;
        let mut interval_stop: i32;

        if interval_length < 0 {
            continue;
        }

        if beacon.1 == target {
            if interval_length == 0 {
                continue;
            } else if beacon.0 == sensor.0 - interval_length {
                interval_start = sensor.0 - interval_length + 1;
                interval_stop = sensor.0 + interval_length;
            } else {
                interval_start = sensor.0 - interval_length;
                interval_stop = sensor.0 + interval_length - 1;
            }
        } else {
            interval_start = sensor.0 - interval_length;
            interval_stop = sensor.0 + interval_length;
        }

        if let Some((min, max)) = bounds {
            interval_start = interval_start.max(min);
            interval_stop = interval_stop.min(max);
        }

        intervals = intervals.add(Interval::new(interval_start, interval_stop));
    }

    intervals
}

fn problem_one(input: &String, target: i32) -> i32 {
    find_non_beacon_intervals(&parse_input(input), target, None).length()
}

fn problem_two(input: &String) -> u64 {
    let input = parse_input(input);
    let bound = 4000000;
    let units = input
        .iter()
        .flat_map(|(sensor, beacon)| vec![sensor, beacon])
        .collect::<HashSet<_>>();

    for target in 0..=bound {
        let intervals = find_non_beacon_intervals(&input, target, Some((0, bound)));

        if let IntervalUnion::Union(is) = intervals {
            if let &[i1, i2] = &is[..] {
                let intersection = i1.1.min(i2.1) + 1;

                if !units.contains(&Pos(intersection, target)) {
                    return target as u64 + 4000000 * (intersection as u64);
                }
            }
        }
    }

    panic!()
}

fn parse_input(input: &String) -> Vec<(Pos, Pos)> {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();

    input
        .to_vec()
        .iter()
        .map(|s| {
            let captures = re.captures(s).unwrap();
            (
                Pos(
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                Pos(
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn main() {
    let input = read_input(15);
    println!("Problem one: {}", problem_one(&input, 2000000));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string(), 10), 26);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 56000011);
    }
}
