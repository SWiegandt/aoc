use std::{collections::HashMap, fmt::Display};

use aoc2022::util::{read_input, ToInputVec};

#[derive(Clone, Debug)]
enum Material {
    Air,
    Rock,
    Sand,
    Source,
    Floor,
}

struct Map(HashMap<(i32, i32), Material>);

impl Map {
    fn bounds(&self) -> ((i32, i32), (i32, i32)) {
        let ys = self.0.keys().map(|&(y, _)| y).collect::<Vec<_>>();
        let xs = self.0.keys().map(|&(_, x)| x).collect::<Vec<_>>();

        (
            (*ys.iter().min().unwrap(), *xs.iter().min().unwrap()),
            (*ys.iter().max().unwrap(), *xs.iter().max().unwrap()),
        )
    }
}

impl Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Material::Air => ".",
                Material::Rock => "#",
                Material::Sand => "o",
                Material::Source => "+",
                Material::Floor => "#",
            }
        )
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ((y_min, x_min), (y_max, x_max)) = self.bounds();
        let mut output = "".to_string();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                output.extend(format!("{}", self.0.get(&(y, x)).unwrap_or(&Material::Air)).chars());
            }

            output.extend(['\n']);
        }

        write!(f, "{}", output)
    }
}

fn problem_one(input: &String) -> i32 {
    let mut map = parse_input(input);
    let mut sands_at_rest = 0;
    let max_y = map.keys().map(|(y, _)| y).max().unwrap().clone();
    let mut last_path = vec![(0, 500)];

    'outer: loop {
        let (mut y, mut x) = last_path.pop().unwrap();

        loop {
            last_path.push((y, x));

            if y >= max_y {
                break 'outer;
            }

            if !map.contains_key(&(y + 1, x)) {
                y += 1;
            } else if !map.contains_key(&(y + 1, x - 1)) {
                y += 1;
                x -= 1;
            } else if !map.contains_key(&(y + 1, x + 1)) {
                y += 1;
                x += 1;
            } else {
                map.insert((y, x), Material::Sand);
                sands_at_rest += 1;
                last_path.pop();
                break;
            }
        }
    }

    sands_at_rest
}

fn problem_two(input: &String) -> i32 {
    let mut map = parse_input(input);
    let mut sands_at_rest = 0;
    let max_y = map.keys().map(|(y, _)| y).max().unwrap().clone();
    let mut last_path = vec![(0, 500)];

    'outer: loop {
        let (mut y, mut x) = last_path.pop().unwrap();

        loop {
            last_path.push((y, x));

            if y == max_y + 1 {
                for x in x - 1..=x + 1 {
                    map.entry((y + 1, x)).or_insert(Material::Floor);
                }
            }

            if !map.contains_key(&(y + 1, x)) {
                y += 1;
            } else if !map.contains_key(&(y + 1, x - 1)) {
                y += 1;
                x -= 1;
            } else if !map.contains_key(&(y + 1, x + 1)) {
                y += 1;
                x += 1;
            } else if let Some(Material::Source) = map.get(&(y, x)) {
                map.insert((y, x), Material::Sand);
                sands_at_rest += 1;
                break 'outer;
            } else {
                map.insert((y, x), Material::Sand);
                sands_at_rest += 1;
                last_path.pop();
                break;
            }
        }
    }

    sands_at_rest
}

fn parse_input(input: &String) -> HashMap<(i32, i32), Material> {
    let input: Vec<Vec<(i32, i32)>> = input
        .to_vec()
        .iter()
        .map(|s| {
            let coords = s.split(" -> ");
            coords
                .map(|c| {
                    let (fst, snd) = c.split_once(",").unwrap();
                    (fst.parse().unwrap(), snd.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let mut map: HashMap<(i32, i32), Material> = HashMap::new();
    map.insert((0, 500), Material::Source);

    for row in input {
        for (&(x1, y1), &(x2, y2)) in row.iter().zip(row[1..].iter()) {
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    map.insert((y, x), Material::Rock);
                }
            }
        }
    }

    map
}

fn main() {
    let input = read_input(14);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 24);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 93);
    }
}
