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
    let mut current_nodes: Vec<(i32, i32)> = vec![];
    let mut distance = 0;

    let mut distances: Map<Option<i32>> = input
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if start(*c) {
                        current_nodes.push((y as i32, x as i32));
                        Some(distance)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let y_len = distances.len();
    let x_len = distances[0].len();

    'outer: loop {
        distance += 1;
        let mut next_nodes = HashSet::new();

        for (y, x) in current_nodes.iter() {
            let elevation = to_elevation(input[*y as usize][*x as usize]);

            if y - 1 >= 0 {
                if to_elevation(input[(y - 1) as usize][*x as usize]) <= elevation + 1 {
                    if let Some(d) = distances[(y - 1) as usize][*x as usize] {
                        if d > distance {
                            next_nodes.insert((y - 1, *x));
                        }
                    } else {
                        next_nodes.insert((y - 1, *x));
                    }
                }
            }

            if ((y + 1) as usize) < y_len {
                if to_elevation(input[(y + 1) as usize][*x as usize]) <= elevation + 1 {
                    if let Some(d) = distances[(y + 1) as usize][*x as usize] {
                        if d > distance {
                            next_nodes.insert((y + 1, *x));
                        }
                    } else {
                        next_nodes.insert((y + 1, *x));
                    }
                }
            }

            if x - 1 >= 0 {
                if to_elevation(input[*y as usize][(x - 1) as usize]) <= elevation + 1 {
                    if let Some(d) = distances[*y as usize][(x - 1) as usize] {
                        if d > distance {
                            next_nodes.insert((*y, (x - 1)));
                        }
                    } else {
                        next_nodes.insert((*y, (x - 1)));
                    }
                }
            }

            if ((x + 1) as usize) < x_len {
                if to_elevation(input[*y as usize][(x + 1) as usize]) <= elevation + 1 {
                    if let Some(d) = distances[*y as usize][(x + 1) as usize] {
                        if d > distance {
                            next_nodes.insert((*y, x + 1));
                        }
                    } else {
                        next_nodes.insert((*y, x + 1));
                    }
                }
            }
        }

        for (y, x) in next_nodes.iter() {
            distances[*y as usize][*x as usize] = Some(distance);

            if input[*y as usize][*x as usize] == 'E' {
                break 'outer distance;
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
