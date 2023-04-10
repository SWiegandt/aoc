use aoc2022::util::{read_input, ToInputVec};

fn visible_from_left(rows: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    rows.iter()
        .map(|row| {
            row.iter()
                .scan(-1, |max_height, tree| {
                    let visible = tree > max_height;
                    *max_height = max_height.clone().max(tree.clone());
                    Some(visible)
                })
                .collect()
        })
        .collect()
}

fn visible_from_right(rows: &Vec<Vec<i32>>) -> Vec<Vec<bool>> {
    let mut from_left_rev = visible_from_left(&rows.iter().map(|row| row.iter().cloned().rev().collect()).collect());
    from_left_rev.iter_mut().for_each(|row| row.reverse());
    from_left_rev
}

fn transpose<T: Clone + Default>(rows: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![vec![Default::default(); rows.len()]; rows[0].len()];

    for (y, row) in rows.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            transposed[x][y] = val.clone();
        }
    }

    transposed
}

fn num_visible_to_left(row: &Vec<i32>, index: usize) -> usize {
    for n in (0..index).rev() {
        if row[n] >= row[index] {
            return index - n;
        }
    }

    index
}

fn num_visible_to_right(row: &Vec<i32>, index: usize) -> usize {
    for n in index + 1..row.len() {
        if row[n] >= row[index] {
            return n - index;
        }
    }

    row.len() - index - 1
}

fn problem_one(input: &String) -> i32 {
    let input = parse_input(input);
    let transposed = transpose(&input);
    let left = visible_from_left(&input);
    let right = visible_from_right(&input);
    let top = visible_from_left(&transposed);
    let bottom = visible_from_right(&transposed);
    let mut visible = 0;

    for x in 0..input.len() {
        for y in 0..input[0].len() {
            if left[x][y] || right[x][y] || top[y][x] || bottom[y][x] {
                visible += 1;
            }
        }
    }

    visible
}

fn problem_two(input: &String) -> i32 {
    let input = parse_input(input);
    let transposed = transpose(&input);
    let mut max_scenic = 0;

    for (n, row) in input.iter().enumerate() {
        for index in 0..row.len() {
            let left = num_visible_to_left(row, index);
            let right = num_visible_to_right(row, index);
            let top = num_visible_to_left(&transposed[index], n);
            let bottom = num_visible_to_right(&transposed[index], n);
            max_scenic = max_scenic.max(left * right * top * bottom);
        }
    }

    max_scenic as i32
}

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    input
        .to_vec()
        .iter()
        .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect()
}

fn main() {
    let input = read_input(8);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 21);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), 8);
    }
}
