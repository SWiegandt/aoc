use std::collections::HashSet;

use regex::Regex;

use crate::util;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn to_interval(&self) -> Interval {
        Interval(self.0, self.0)
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

fn one(input: &Vec<(Pos, Pos)>) -> i32 {
    find_non_beacon_intervals(input, 2000000, None).length()
}

fn two(input: &Vec<(Pos, Pos)>) -> u64 {
    let bound = 4000000;
    let units = input
        .iter()
        .flat_map(|(sensor, beacon)| vec![sensor, beacon])
        .collect::<HashSet<_>>();

    for target in 0..=bound {
        let intervals = find_non_beacon_intervals(input, target, Some((0, bound)));

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

pub fn run() -> (i32, u64) {
    let re = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();

    let input: Vec<(Pos, Pos)> = util::read_input(15)
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
        .collect();

    (one(&input), two(&input))
}
