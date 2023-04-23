use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc2022::util::read_input;

type Point = (i32, i32);
type Map = HashMap<Point, Vec<Direction>>;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_string(chr: char) -> Option<Direction> {
        match chr {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            _ => None,
        }
    }

    fn tuple(&self) -> (i32, i32) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "<",
                Direction::Right => ">",
                Direction::Up => "^",
                Direction::Down => "v",
            }
        )
    }
}

struct MapWrapper<'a>(&'a Map);

impl<'a> MapWrapper<'a> {
    fn dimensions(&self) -> (i32, i32) {
        (
            self.0.keys().map(|&(x, _)| x).max().unwrap(),
            self.0.keys().map(|&(_, y)| y).max().unwrap(),
        )
    }
}

fn parse_input(input: &String) -> HashMap<Point, Vec<Direction>> {
    let mut map: HashMap<Point, Vec<Direction>> = HashMap::new();

    for (y, row) in input.lines().enumerate() {
        for (x, chr) in row.chars().enumerate() {
            if let Some(direction) = Direction::from_string(chr) {
                map.entry((x as i32, y as i32))
                    .and_modify(|ls| ls.push(direction.clone()))
                    .or_insert(vec![direction]);
            }
        }
    }

    map
}

fn print_map(map: &Map, map_width: i32, map_height: i32) {
    println!("#.{}", "#".repeat(map_width as usize));

    for y in 1..=map_height {
        print!("#");

        for x in 1..=map_width {
            if let Some(ls) = map.get(&(x, y)) {
                if ls.len() > 1 {
                    print!("{}", ls.len());
                } else {
                    print!("{}", ls[0]);
                }
            } else {
                print!(".");
            }
        }

        print!("#\n");
    }

    println!("{}.#\n", "#".repeat(map_width as usize));
}

fn mutate_map(map: &mut Map, map_width: i32, map_height: i32) {
    let mut updated_map: Map = HashMap::new();

    for ((x, y), directions) in map.iter() {
        for direction in directions {
            let (xm, ym) = direction.tuple();

            let new_x = if x + xm <= 0 {
                map_width
            } else if x + xm > map_width {
                1
            } else {
                x + xm
            };

            let new_y = if y + ym <= 0 {
                map_height
            } else if y + ym > map_height {
                1
            } else {
                y + ym
            };

            updated_map
                .entry((new_x, new_y))
                .and_modify(|ls| ls.push(direction.clone()))
                .or_insert(vec![direction.clone()]);
        }
    }

    *map = updated_map;
}

fn find_exit(map: &mut Map, mut goals: Vec<Point>) -> i32 {
    let mut stack: HashSet<Point> = HashSet::from([(1, 0)]);
    let (map_width, map_height) = MapWrapper(&map).dimensions();
    let mut goal = goals.pop().unwrap();

    for cycle in 0.. {
        mutate_map(map, map_width, map_height);
        let mut next_stack = HashSet::new();

        for &(x, y) in stack.iter() {
            if (x, y) == goal {
                if let Some(new_goal) = goals.pop() {
                    next_stack = HashSet::from([(x, y)]);
                    goal = new_goal;
                    break;
                } else {
                    return cycle;
                }
            }

            for (x, y) in vec![(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if map.get(&(x, y)).unwrap_or(&vec![]).is_empty()
                    && (((1..=map_width).contains(&x) && (1..=map_height).contains(&y))
                        || (y == 0 && x == 1)
                        || (y == map_height + 1 && x == map_width))
                {
                    next_stack.insert((x, y));
                }
            }
        }

        stack = next_stack;
    }

    unreachable!()
}

fn problem_one(input: &String) -> i32 {
    let mut map = parse_input(input);
    let (map_width, map_height) = MapWrapper(&map).dimensions();
    let goals = vec![(map_width, map_height + 1)];
    find_exit(&mut map, goals)
}

fn problem_two(input: &String) -> i32 {
    let mut map = parse_input(input);
    let (map_width, map_height) = MapWrapper(&map).dimensions();
    let goals = vec![(map_width, map_height + 1), (1, 0), (map_width, map_height + 1)];
    find_exit(&mut map, goals)
}

fn main() {
    let input = read_input(24);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 18);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 54);
    }
}
