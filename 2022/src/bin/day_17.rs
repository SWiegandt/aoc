use std::{collections::HashSet, fmt::Display};

use aoc2022::util::read_input;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos(i64, i64);

impl Pos {
    fn translate(&mut self, Pos(x, y): &Pos) {
        self.0 += x;
        self.1 += y;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Shape {
    Line,
    Plus,
    Angle,
    Bar,
    Square,
}

impl Shape {
    fn sprite(&self) -> Vec<Pos> {
        match self {
            Shape::Line => vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(3, 0)],
            Shape::Plus => vec![Pos(1, 0), Pos(0, 1), Pos(1, 1), Pos(2, 1), Pos(1, 2)],
            Shape::Angle => vec![Pos(0, 0), Pos(1, 0), Pos(2, 0), Pos(2, 1), Pos(2, 2)],
            Shape::Bar => vec![Pos(0, 0), Pos(0, 1), Pos(0, 2), Pos(0, 3)],
            Shape::Square => vec![Pos(0, 0), Pos(0, 1), Pos(1, 0), Pos(1, 1)],
        }
    }

    fn translate(&self, pos: &Pos) -> Vec<Pos> {
        let mut sprite = self.sprite();

        for sp in sprite.iter_mut() {
            sp.translate(pos);
        }

        sprite
    }
}

struct GameArea<'a>(&'a Vec<Pos>);

impl<'a> Display for GameArea<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = 6;
        let min_y = self.0.iter().map(|&Pos(_, y)| y).min().unwrap();
        let max_y = self.0.iter().map(|&Pos(_, y)| y).max().unwrap();
        let mut output = vec![];

        for y in (min_y..=max_y).rev() {
            let mut row = vec![];

            for x in 0..=max_x {
                if self.0.contains(&Pos(x, y)) {
                    row.push("#");
                } else {
                    row.push(".");
                }
            }

            output.push(row.join(""));
        }

        write!(f, "{}", output.join("\n"))
    }
}

enum Move {
    Left,
    Right,
}

impl Move {
    fn x(&self) -> i64 {
        match self {
            Move::Left => -1,
            Move::Right => 1,
        }
    }
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '<' => Move::Left,
            '>' => Move::Right,
            _ => panic!(),
        }
    }
}

fn problem_one_and_two(input: &String, count_cycles: bool) -> i64 {
    let input: Vec<Move> = input.trim().chars().map(|c| c.into()).collect();
    let falling_shapes = [Shape::Line, Shape::Plus, Shape::Angle, Shape::Bar, Shape::Square]
        .iter()
        .cycle();

    let mut game_area: Vec<Pos> = vec![];
    let mut starting_pos = Pos(2, 3);
    let mut wind_cycle = input.iter().enumerate().cycle();
    let mut cycle_states: HashSet<(i64, usize, Shape)> = HashSet::new();
    let mut cycle_saved_states: Vec<(i64, usize, Shape)> = vec![];
    let mut cycle_start_height: Option<i64> = None;
    let mut cycle_start_index: Option<usize> = None;
    let mut cycle_post_heights: Vec<i64> = vec![];

    let falling_shapes: Box<dyn Iterator<Item = (usize, &Shape)>> = if !count_cycles {
        Box::new(falling_shapes.take(2022).enumerate())
    } else {
        Box::new(falling_shapes.enumerate())
    };

    for (shape_index, shape) in falling_shapes {
        let mut pos = starting_pos.clone();

        'outer: loop {
            let (wind_index, wind) = wind_cycle.next().unwrap();
            let x_translation = wind.x();
            pos.translate(&Pos(x_translation, 0));

            for sp in shape.translate(&pos) {
                if sp.0 < 0 || sp.0 > 6 || game_area.contains(&sp) {
                    pos.translate(&Pos(-x_translation, 0));
                    break;
                }
            }

            pos.translate(&Pos(0, -1));

            for sp in shape.translate(&pos) {
                if sp.1 >= 0 && !game_area.contains(&sp) {
                    continue;
                }

                pos.translate(&Pos(0, 1));
                let translated_shape = shape.translate(&pos);
                let shape_top = translated_shape.iter().map(|Pos(_, y)| y).max().unwrap();
                starting_pos.1 = starting_pos.1.max(shape_top + 4);

                if count_cycles {
                    let cycle_state = (pos.0, wind_index, shape.clone());

                    if cycle_states.contains(&cycle_state) {
                        if cycle_saved_states.contains(&cycle_state) && cycle_saved_states.len() > 25 {
                            let num_shapes = 1000000000000;
                            let cycle_length = shape_index as i64 - cycle_start_index.unwrap() as i64;
                            let cycles_left = (num_shapes - cycle_start_index.unwrap() as i64) / cycle_length;
                            let cycle_height =
                                game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1 - cycle_start_height.unwrap();

                            let final_height = cycles_left as i64 * cycle_height
                                + if num_shapes - cycles_left * cycle_length - cycle_start_index.unwrap() as i64 >= 0 {
                                    cycle_post_heights[(num_shapes
                                        - cycles_left * cycle_length
                                        - cycle_start_index.unwrap() as i64)
                                        as usize]
                                } else {
                                    cycle_start_height.unwrap()
                                };

                            return final_height;
                        } else {
                            cycle_post_heights.push(game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1);
                        }

                        cycle_saved_states.push(cycle_state.clone());
                        cycle_start_height =
                            cycle_start_height.or(Some(game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1));
                        cycle_start_index = cycle_start_index.or(Some(shape_index));
                    } else {
                        cycle_post_heights.clear();
                        cycle_saved_states.clear();
                        cycle_start_height = None;
                        cycle_start_index = None;
                    }

                    cycle_states.insert(cycle_state);
                }

                break 'outer;
            }
        }

        for sp in shape.translate(&pos) {
            game_area.push(sp.clone());
        }
    }

    game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1
}

fn main() {
    let input = read_input(17);
    println!("Problem one: {}", problem_one_and_two(&input, false));
    println!("Problem two: {}", problem_one_and_two(&input, true));
}

mod tests {
    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one_and_two(&TEST_INPUT.trim().to_string(), false), 3068);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(
            super::problem_one_and_two(&TEST_INPUT.trim().to_string(), true),
            1514285714288
        );
    }
}
