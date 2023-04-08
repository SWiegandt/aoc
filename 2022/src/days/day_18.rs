use std::collections::HashSet;

use crate::util;

type Pos = (usize, usize, usize);

fn get_size(input: &Vec<Vec<usize>>) -> usize {
    *input.iter().map(|row| row.iter().max().unwrap()).max().unwrap() + 3
}

fn init_lava(input: &Vec<Vec<usize>>, lava_size: usize) -> Vec<Vec<Vec<i32>>> {
    let mut lava = vec![vec![vec![0; lava_size]; lava_size]; lava_size];

    for row in input {
        if let [x, y, z] = &row[..] {
            lava[*x + 1][*y + 1][*z + 1] = 1;
        }
    }

    lava
}

fn get_area(lava: &Vec<Vec<Vec<i32>>>, lava_size: usize, (x, y, z): Pos) -> i32 {
    (1 - lava[x][y][z])
        * (if x > 0 { lava[x - 1][y][z] } else { 0 }
            + if x < lava_size - 1 { lava[x + 1][y][z] } else { 0 }
            + if y > 0 { lava[x][y - 1][z] } else { 0 }
            + if y < lava_size - 1 { lava[x][y + 1][z] } else { 0 }
            + if z > 0 { lava[x][y][z - 1] } else { 0 }
            + if z < lava_size - 1 { lava[x][y][z + 1] } else { 0 })
}

fn one(input: &Vec<Vec<usize>>) -> i32 {
    let mut area = 0;
    let lava_size = get_size(&input);
    let lava = init_lava(input, lava_size);

    for x in 0..lava_size {
        for y in 0..lava_size {
            for z in 0..lava_size {
                area += get_area(&lava, lava_size, (x, y, z));
            }
        }
    }

    area
}

fn two(input: &Vec<Vec<usize>>) -> i32 {
    let mut area = 0;
    let lava_size = get_size(&input);
    let lava = init_lava(input, lava_size);
    let mut next_loop: HashSet<Pos> = HashSet::from([(0, 0, 0)]);
    let mut visited = HashSet::new();

    loop {
        let current: HashSet<Pos> = next_loop.clone();
        next_loop.clear();

        if current.is_empty() {
            break;
        }

        for (x, y, z) in current {
            visited.insert((x, y, z));
            area += get_area(&lava, lava_size, (x, y, z));

            for pos in [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            .into_iter()
            .filter(|(x, y, z)| [x, y, z].iter().all(|n| (0..lava_size).contains(n)))
            .filter(|pos| !visited.contains(pos))
            .filter(|&(x, y, z)| lava[x][y][z] == 0)
            {
                if !next_loop.contains(&pos) {
                    next_loop.insert(pos);
                }
            }
        }
    }

    area
}

pub fn run() -> (i32, i32) {
    let input: Vec<Vec<usize>> = util::read_input(18)
        .iter()
        .map(|row| {
            row.split(",")
                .map(|s| s.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (one(&input), two(&input))
}
