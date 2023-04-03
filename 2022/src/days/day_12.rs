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
            break 'outer distance;
        }
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
