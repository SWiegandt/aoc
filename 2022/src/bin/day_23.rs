use std::collections::{HashMap, HashSet};

use aoc2022::util::read_input;

type Map = HashMap<(i32, i32), ()>;

fn parse_input(input: &String) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, char)| {
                if char == '#' {
                    Some(((x as i32, y as i32), ()))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn bounding_rectangle(map: &Map) -> ((i32, i32), (i32, i32), i32) {
    let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

    let elves_in_rectangle = map
        .keys()
        .filter(|&(x, y)| (min_x..=max_x).contains(x) && (min_y..=max_y).contains(y))
        .count() as i32;

    ((min_x, max_x), (min_y, max_y), elves_in_rectangle)
}

fn print_map((min_x, max_x): (i32, i32), (min_y, max_y): (i32, i32), map: &Map) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if map.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        print!("\n");
    }

    print!("\n");
}

fn move_elves(map: &mut Map, cycle: usize) -> bool {
    let movements = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut considered_moves: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut alone_elves: HashSet<(i32, i32)> = map.keys().cloned().collect();

    for ((x, y), _) in map.iter() {
        for xm in -1..2 {
            for ym in -1..2 {
                if (xm, ym) == (0, 0) {
                    continue;
                } else if map.contains_key(&(x + xm, y + ym)) {
                    alone_elves.remove(&(*x, *y));
                }
            }
        }

        if alone_elves.contains(&(*x, *y)) {
            continue;
        }

        for (xm, ym) in movements.iter().cycle().skip(cycle).take(4) {
            if !(map.contains_key(&(x + xm, y + ym))
                || map.contains_key(&(x + xm + ym.abs(), y + ym + xm.abs()))
                || map.contains_key(&(x + xm - ym.abs(), y + ym - xm.abs())))
            {
                if considered_moves.contains_key(&(x + xm, y + ym)) {
                    considered_moves.insert((x + xm, y + ym), None);
                } else {
                    considered_moves.insert((x + xm, y + ym), Some((*x, *y)));
                }

                break;
            }
        }
    }

    if alone_elves.len() == map.len() {
        return true;
    }

    for (new, old) in considered_moves.iter().filter(|&(_, o)| o.is_some()) {
        map.remove(&old.unwrap());
        map.insert(*new, ());
    }

    false
}

fn problem_one(input: &String) -> i32 {
    let mut map = parse_input(input);

    for (i, cycle) in (0..4).cycle().enumerate() {
        if i >= 10 {
            break;
        }

        move_elves(&mut map, cycle);
    }

    let ((min_x, max_x), (min_y, max_y), elves) = bounding_rectangle(&map);
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves
}

fn problem_two(input: &String) -> usize {
    let mut map = parse_input(input);

    for (i, cycle) in (0..4).cycle().enumerate() {
        if move_elves(&mut map, cycle) {
            return i + 1;
        }
    }

    panic!()
}

fn main() {
    let input = read_input(23);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 110);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 20);
    }
}
