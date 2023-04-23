use std::{
    collections::HashMap,
    fmt::Display,
    ops::{MulAssign, Neg},
};

use aoc2022::util::read_input;
use regex::Regex;

#[derive(Debug, Clone)]
enum Tile {
    Open,
    Wall,
}

impl Tile {
    fn from_char(chr: char) -> Option<Tile> {
        match chr {
            '.' => Some(Tile::Open),
            '#' => Some(Tile::Wall),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Direction(i32, i32);

impl Direction {
    fn facing(&self) -> i64 {
        match self {
            Direction(0, 1) => 0,
            Direction(1, 0) => 1,
            Direction(0, -1) => 2,
            Direction(-1, 0) => 3,
            _ => unreachable!(),
        }
    }

    const Y: Direction = Direction(1, 0);
    const X: Direction = Direction(0, 1);
}

impl MulAssign for Direction {
    fn mul_assign(&mut self, rhs: Self) {
        let new = Self(self.1 * rhs.0 + self.0 * rhs.1, self.1 * rhs.1 - self.0 * rhs.0);
        self.0 = new.0;
        self.1 = new.1;
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction(y, x) => Direction(-y, -x),
        }
    }
}

#[derive(Debug)]
enum Move {
    Turn(Direction),
    Walk(i32),
}

type Point = (usize, usize);
type Map = Vec<Vec<Option<Tile>>>;

#[derive(Debug)]
struct Sprite(Point, Direction);

impl Display for Sprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.1 {
                Direction(1, 0) => "v",
                Direction(0, 1) => ">",
                Direction(0, -1) => "<",
                Direction(-1, 0) => "^",
                _ => unreachable!(),
            }
        )
    }
}

fn print_map(map: &Map, sprite: &Sprite) {
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if sprite.0 == (y, x) {
                print!("{}", sprite);
            } else {
                print!(
                    "{}",
                    match tile {
                        Some(Tile::Open) => ".",
                        Some(Tile::Wall) => "#",
                        _ => " ",
                    }
                )
            }
        }

        print!("\n");
    }
}

fn parse_input(input: &String) -> (Map, Vec<Move>, Sprite) {
    let mut iter = input.lines().into_iter();
    let move_string = iter.nth_back(0).unwrap();
    iter.nth_back(0);

    let mut map = vec![vec![]];
    let mut moves = vec![];
    let re = Regex::new(r"((\d+)|([A-Z]))").unwrap();
    let mut starting_point: Option<Point> = None;

    for (y, line) in iter.enumerate() {
        map.push(vec![]);

        for (x, chr) in line.chars().enumerate() {
            let tile = Tile::from_char(chr);

            if tile.is_some() {
                starting_point = starting_point.or(Some((y, x)));
            }

            map[y].push(tile);
        }
    }

    for captures in re.captures_iter(move_string) {
        moves.push(if let Some(capture) = captures.get(2) {
            Move::Walk(capture.as_str().parse().unwrap())
        } else if let Some(capture) = captures.get(3) {
            Move::Turn(match capture.as_str() {
                "L" => -Direction::Y,
                "R" => Direction::Y,
                _ => unreachable!(),
            })
        } else {
            unreachable!()
        });
    }

    (map, moves, Sprite(starting_point.unwrap(), Direction::X))
}

fn problem_one(input: &String) -> i64 {
    let (map, moves, mut sprite) = parse_input(input);

    for mv in moves {
        if let Move::Turn(direction) = mv {
            sprite.1 *= direction;
        } else if let Move::Walk(count) = mv {
            'outer: for _ in 0..count {
                let mut to = (sprite.0 .0 as i32 + sprite.1 .0, sprite.0 .1 as i32 + sprite.1 .1);

                loop {
                    if to.0 < 0 {
                        to.0 = map.len() as i32;
                    } else if to.0 >= map.len() as i32 {
                        to.0 = 0;
                    } else if to.1 < 0 {
                        to.1 = map[sprite.0 .0].len() as i32;
                    } else if to.1 >= map[sprite.0 .0].len() as i32 {
                        to.1 = 0;
                    }

                    if let Some(tile) = map
                        .get(to.0 as usize)
                        .and_then(|row| row.get(to.1 as usize).cloned())
                        .flatten()
                    {
                        if let Tile::Open = tile {
                            sprite.0 = (to.0 as usize, to.1 as usize);
                        } else if let Tile::Wall = tile {
                            break 'outer;
                        }

                        break;
                    } else {
                        to.0 += sprite.1 .0;
                        to.1 += sprite.1 .1;
                    }
                }
            }
        }
    }

    1000 * (sprite.0 .0 as i64 + 1) + 4 * (sprite.0 .1 as i64 + 1) + sprite.1.facing()
}

fn problem_two(input: &String, boundaries: &HashMap<(i32, i32, Direction), (i32, i32, Direction)>) -> i64 {
    let (map, moves, mut sprite) = parse_input(input);

    for mv in moves {
        if let Move::Turn(direction) = mv {
            sprite.1 *= direction;
        } else if let Move::Walk(count) = mv {
            'outer: for _ in 0..count {
                let mut to = (sprite.0 .0 as i32 + sprite.1 .0, sprite.0 .1 as i32 + sprite.1 .1);
                let mut direction: Direction = sprite.1.clone();

                if let Some(boundary) = boundaries.get(&(to.0, to.1, sprite.1.clone())) {
                    to = (boundary.0, boundary.1);
                    direction = boundary.2.clone();
                }

                if let Some(tile) = map
                    .get(to.0 as usize)
                    .and_then(|row| row.get(to.1 as usize).cloned())
                    .flatten()
                {
                    if let Tile::Open = tile {
                        sprite.0 = (to.0 as usize, to.1 as usize);
                        sprite.1 = direction;
                    } else if let Tile::Wall = tile {
                        break 'outer;
                    }
                }
            }
        }
    }

    1000 * (sprite.0 .0 as i64 + 1) + 4 * (sprite.0 .1 as i64 + 1) + sprite.1.facing()
}

fn main() {
    let input = read_input(22);
    println!("Problem one: {:?}", problem_one(&input));

    let mut boundaries = HashMap::new();
    let size = 50;

    // 3 -> 4
    boundaries.extend((size..2 * size).map(|y| ((y, size - 1, -Direction::X), (2 * size, y - size, Direction::Y))));
    boundaries.extend((size..2 * size).map(|x| ((2 * size - 1, x - size, -Direction::Y), (x, size, Direction::X))));

    // 3 -> 2
    boundaries.extend((size..2 * size).map(|y| ((y, 2 * size, Direction::X), (size - 1, size + y, -Direction::Y))));
    boundaries.extend((size..2 * size).map(|x| ((size, size + x, Direction::Y), (x, 2 * size - 1, -Direction::X))));

    // 1 -> 6
    boundaries.extend((size..2 * size).map(|x| ((-1, x, -Direction::Y), (2 * size + x, 0, Direction::X))));
    boundaries.extend((size..2 * size).map(|y| ((2 * size + y, -1, -Direction::X), (0, y, Direction::Y))));

    // 1 -> 4
    boundaries.extend((0..size).map(|y| ((y, size - 1, -Direction::X), (3 * size - 1 - y, 0, Direction::X))));
    boundaries.extend((0..size).map(|y| ((3 * size - 1 - y, -1, -Direction::X), (y, size, Direction::X))));

    // 5 -> 6
    boundaries.extend((size..2 * size).map(|x| ((3 * size, x, Direction::Y), (2 * size + x, size - 1, -Direction::X))));
    boundaries.extend((size..2 * size).map(|y| ((2 * size + y, size, Direction::X), (3 * size - 1, y, -Direction::Y))));

    // 5 -> 2
    boundaries.extend((2 * size..3 * size).map(|y| {
        (
            (y, 2 * size, Direction::X),
            (3 * size - 1 - y, 3 * size - 1, -Direction::X),
        )
    }));

    boundaries.extend((2 * size..3 * size).map(|y| {
        (
            (3 * size - 1 - y, 3 * size, Direction::X),
            (y, 2 * size - 1, -Direction::X),
        )
    }));

    // 2 -> 6
    boundaries
        .extend((2 * size..3 * size).map(|x| ((-1, x, -Direction::Y), (4 * size - 1, x - 2 * size, -Direction::Y))));

    boundaries.extend((2 * size..3 * size).map(|x| ((4 * size, x - 2 * size, Direction::Y), (0, x, Direction::Y))));

    println!("Problem two: {:?}", problem_two(&input, &boundaries));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Direction;

    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 6032);
    }

    #[test]
    fn test_problem_two() {
        let mut boundaries = HashMap::new();

        // 1 -> 2
        boundaries.extend((0..4).map(|x| ((-1, 2 * 4 + x, -Direction::Y), (4, 4 - x - 1, Direction::Y))));

        boundaries.extend((0..4).map(|x| ((4 - 1, 4 - x - 1, -Direction::Y), (0, 2 * 4 + x, Direction::Y))));

        // 1 -> 3
        boundaries.extend((0..4).map(|y| ((y, 2 * 4 - 1, -Direction::X), (4, 4 + y, Direction::Y))));

        boundaries.extend((0..4).map(|x| ((4 - 1, 4 + x, -Direction::Y), (x, 2 * 4, Direction::X))));

        // 1 -> 6
        boundaries.extend((0..4).map(|y| ((y, 3 * 4, Direction::X), (3 * 4 - y - 1, 4 * 4 - 1, -Direction::X))));

        boundaries.extend((0..4).map(|y| ((3 * 4 - y - 1, 4 * 4, Direction::X), (y, 3 * 4 - 1, -Direction::X))));

        // 5 -> 2
        boundaries.extend((0..4).map(|x| ((3 * 4, 2 * 4 + x, Direction::Y), (2 * 4 - 1, 4 - x - 1, -Direction::Y))));

        boundaries.extend((0..4).map(|x| ((2 * 4, 4 - x - 1, Direction::Y), (3 * 4 - 1, 2 * 4 + x, -Direction::Y))));

        // 5 -> 3
        boundaries.extend((0..4).map(|y| {
            (
                (2 * 4 + y, 2 * 4 - 1, -Direction::X),
                (2 * 4 - 1, 2 * 4 - 1 - y, -Direction::Y),
            )
        }));

        boundaries.extend((0..4).map(|x| ((2 * 4, 2 * 4 - 1 - x, Direction::Y), (2 * 4 + x, 2 * 4, Direction::X))));

        // 6 -> 4
        boundaries.extend((0..4).map(|x| {
            (
                (2 * 4 - 1, 3 * 4 + x, -Direction::Y),
                (2 * 4 - x - 1, 3 * 4 - 1, -Direction::X),
            )
        }));

        boundaries.extend((0..4).map(|y| ((2 * 4 - y - 1, 3 * 4, Direction::X), (2 * 4, 3 * 4 + y, Direction::Y))));

        assert_eq!(super::problem_two(&TEST_INPUT.to_string(), &boundaries), 5031);
    }
}
