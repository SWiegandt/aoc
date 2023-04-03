use std::collections::HashSet;

use crate::util;

type Map<T> = Vec<Vec<T>>;

fn to_elevation(c: char) -> i32 {
    match c {
        'S' => to_elevation('a'),
        'E' => to_elevation('z'),
        _ => c as i32,
    }
}

fn find_path<F: Fn(char) -> bool>(input: &Map<char>, start: F) -> i32 {
    let mut current_nodes: Vec<(usize, usize)> = vec![];
    let mut distance = 0;

    let mut distances: Map<Option<i32>> = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if start(*c) {
                        current_nodes.push((y, x));
                        Some(distance)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let y_len = distances.len();

    'outer: loop {
        distance += 1;
        let mut next_nodes = HashSet::new();

        for (y, x) in current_nodes.iter() {
            if input[*y][*x] == 'E' {
                break 'outer distance - 1;
            }

            let elevation = to_elevation(input[*y][*x]);
            let x_len = distances[*y].len();

            for y_neighbor in [y.checked_sub(1), y.checked_add(1)]
                .iter()
                .filter_map(|&y| y.filter(|&y| y < y_len))
            {
                if to_elevation(input[y_neighbor][*x]) <= elevation + 1 {
                    if distances[y_neighbor][*x].is_none() {
                        next_nodes.insert((y_neighbor, *x));
                        distances[y_neighbor][*x] = Some(distance);
                    }
                }
            }

            for x_neighbor in [x.checked_sub(1), x.checked_add(1)]
                .iter()
                .filter_map(|&x| x.filter(|&x| x < x_len))
            {
                if to_elevation(input[*y][x_neighbor]) <= elevation + 1 {
                    if distances[*y][x_neighbor].is_none() {
                        next_nodes.insert((*y, x_neighbor));
                        distances[*y][x_neighbor] = Some(distance);
                    }
                }
            }
        }

        current_nodes = next_nodes.into_iter().collect::<Vec<_>>();
    }
}

fn one(input: &Map<char>) -> i32 {
    find_path(input, |c| c == 'S')
}

fn two(input: &Map<char>) -> i32 {
    find_path(input, |c| c == 'S' || c == 'a')
}

pub fn run() -> (i32, i32) {
    let input = util::read_input(12).iter().map(|s| s.chars().collect()).collect();
    (one(&input), two(&input))
}
