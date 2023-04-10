use aoc2022::util::{read_input, ToInputVec};
use regex::Regex;

#[derive(Debug, Clone)]
struct Crate(char);

#[derive(Debug)]
struct Move(usize, usize, usize);

type Crates = Vec<Vec<Crate>>;

fn crate_row(row: &String) -> Vec<Option<Crate>> {
    row.chars()
        .zip((1..=4).cycle())
        .filter_map(|(c, n)| match n {
            2 if c == ' ' => Some(None),
            2 => Some(Some(Crate(c))),
            _ => None,
        })
        .collect()
}

fn crates(input: &Vec<String>) -> Crates {
    let rows = if let Some((_, rows)) = input
        .iter()
        .take_while(|&row| !row.is_empty())
        .collect::<Vec<_>>()
        .split_last()
    {
        rows.iter().map(|&row| crate_row(row)).collect()
    } else {
        vec![]
    };

    let mut columns: Crates = vec![vec![]; rows.iter().map(Vec::len).max().unwrap()];

    for row in rows.into_iter() {
        for (col, _crate) in row.into_iter().enumerate() {
            if let Some(cr) = _crate {
                columns[col].push(cr);
            }
        }
    }

    columns
}

fn moves(input: &Vec<String>) -> Vec<Move> {
    let move_pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    input
        .iter()
        .skip_while(|row| !row.is_empty())
        .skip(1)
        .map(|row| {
            let groups = move_pattern.captures(row).unwrap();
            Move(
                groups.get(1).unwrap().as_str().parse().unwrap(),
                groups.get(2).unwrap().as_str().parse().unwrap(),
                groups.get(3).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn execute_move(crates: &mut Crates, Move(num, from, to): Move) -> &Crates {
    for _ in 1..=num {
        let from_column = crates.get_mut(from - 1).unwrap();
        let crate_to_move = from_column[0].clone();
        *from_column = from_column[1..].to_vec();

        let to_column = crates.get_mut(to - 1).unwrap();
        *to_column = [&[crate_to_move], to_column.as_slice()].concat();
    }

    crates
}

fn execute_multiple_moves(crates: &mut Crates, Move(num, from, to): Move) -> &Crates {
    let from_column = crates.get_mut(from - 1).unwrap();
    let crates_to_move = from_column[0..num].to_vec();
    *from_column = from_column[num..].to_vec();

    let to_column = crates.get_mut(to - 1).unwrap();
    *to_column = [crates_to_move, (*to_column.clone()).to_vec()].concat();

    crates
}

fn run_moves<F>(input: &Vec<String>, move_fn: F) -> String
where
    F: Fn(&mut Crates, Move) -> &Crates,
{
    let mut crates = crates(input);
    let moves = moves(input);

    for mv in moves {
        move_fn(&mut crates, mv);
    }

    crates.iter().map(|col| col.get(0).unwrap().0).collect::<String>()
}

fn problem_one(input: &String) -> String {
    run_moves(&input.to_vec(), execute_move)
}

fn problem_two(input: &String) -> String {
    run_moves(&input.to_vec(), execute_multiple_moves)
}

fn main() {
    let input = read_input(5);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), "CMZ");
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), "MCD");
    }
}
