use std::collections::HashSet;

use aoc2022::util::{read_input, ToInputVec};

type Map<T> = Vec<Vec<T>>;

fn to_elevation(c: char) -> i32 {
    match c {
        'S' => to_elevation('a'),
        'E' => to_elevation('z'),
        _ => c as i32,
    }
}

fn update_distances(
    y: usize,
    x: usize,
    input: &Map<char>,
    distances: &mut Map<Option<i32>>,
    next_nodes: &mut HashSet<(usize, usize)>,
    elevation: i32,
    distance: i32,
) {
    if to_elevation(input[y][x]) <= elevation + 1 && distances[y][x].is_none() {
        next_nodes.insert((y, x));
        distances[y][x] = Some(distance);
    }
}

fn find_path<F: Fn(&char) -> bool>(input: &Map<char>, start: F) -> i32 {
    let mut distance = 0;
    let y_len = input.len();

    let mut distances: Map<Option<i32>> = input
        .iter()
        .map(|row| row.iter().map(|c| if start(c) { Some(0) } else { None }).collect())
        .collect();

    let mut current_nodes: Vec<(usize, usize)> = distances
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, d)| d.map(|_| (y, x))))
        .collect();

    loop {
        let mut next_nodes = HashSet::new();
        distance += 1;

        for (y, x) in current_nodes {
            let elevation = to_elevation(input[y][x]);
            let x_len = distances[y].len();

            [y.checked_sub(1), y.checked_add(1)]
                .iter()
                .filter_map(|&y| y.filter(|&y| y < y_len))
                .for_each(|y| update_distances(y, x, &input, &mut distances, &mut next_nodes, elevation, distance));

            [x.checked_sub(1), x.checked_add(1)]
                .iter()
                .filter_map(|&x| x.filter(|&x| x < x_len))
                .for_each(|x| update_distances(y, x, &input, &mut distances, &mut next_nodes, elevation, distance));
        }

        current_nodes = next_nodes.into_iter().collect::<Vec<_>>();

        if current_nodes.iter().any(|&(y, x)| input[y][x] == 'E') {
            break distance;
        }
    }
}

fn parse_input(input: &String) -> Map<char> {
    input.to_vec().iter().map(|row| row.chars().collect()).collect()
}

fn problem_one(input: &String) -> i32 {
    let input = parse_input(input);
    find_path(&input, |&c| c == 'S')
}

fn problem_two(input: &String) -> i32 {
    let input = parse_input(input);
    find_path(&input, |&c| c == 'S' || c == 'a')
}

fn main() {
    let input = read_input(12);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 31);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 29);
    }
}
