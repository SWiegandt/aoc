use std::{collections::HashSet, fmt::Display};

use crate::util;

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

fn get_game_area_top(game_area: &Vec<Pos>) -> Option<Vec<Pos>> {
    let mut columns: Vec<Vec<Pos>> = vec![];

    for col in 0..=6 {
        let col_max = game_area
            .iter()
            .filter_map(|&Pos(x, y)| if x == col as i64 { Some(y) } else { None })
            .max()?;

        columns.push(
            game_area
                .iter()
                .filter_map(|&Pos(x, y)| {
                    if x == col as i64 && y >= col_max {
                        Some(Pos(x, y))
                    } else {
                        None
                    }
                })
                .collect(),
        );
    }

    Some(columns.into_iter().flatten().collect())
}

fn one(input: &Vec<Move>) -> i64 {
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

    for (shape_index, shape) in falling_shapes.enumerate() {
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
                let cycle_state = (pos.0, wind_index, shape.clone());

                if cycle_states.contains(&cycle_state) {
                    if cycle_saved_states.contains(&cycle_state) && cycle_saved_states.len() > 25 {
                        let num_shapes = 1000000000000;
                        let cycle_length = shape_index as i64 - cycle_start_index.unwrap() as i64;
                        let cycles_left = (num_shapes - cycle_start_index.unwrap() as i64) / cycle_length;
                        let cycle_height =
                            game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1 - cycle_start_height.unwrap();

                        println!(
                            "floor(({} - {}) / {}) * {} + [{}]; cycles_left: {}",
                            num_shapes,
                            cycle_start_index.unwrap(),
                            cycle_length,
                            cycle_height,
                            (num_shapes - cycles_left * cycle_length - (cycle_start_index.unwrap() as i64) - 1)
                                as usize,
                            cycles_left
                        );

                        println!(
                            "{}, {}, {}, {}",
                            num_shapes,
                            cycles_left,
                            cycle_length,
                            (cycle_start_index.unwrap() as i64)
                        );

                        let final_height = cycles_left as i64 * cycle_height
                            + if num_shapes - cycles_left * cycle_length - cycle_start_index.unwrap() as i64 >= 0 {
                                cycle_post_heights[(num_shapes
                                    - cycles_left * cycle_length
                                    - cycle_start_index.unwrap() as i64)
                                    as usize]
                            } else {
                                cycle_start_height.unwrap()
                            };

                        println!("{final_height}");

                        return 0;
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
                break 'outer;
            }
        }

        for sp in shape.translate(&pos) {
            game_area.push(sp.clone());
        }
    }

    game_area.iter().map(|&Pos(_, y)| y).max().unwrap() + 1
}

fn two(_input: &Vec<Move>) -> i64 {
    2
}

pub fn run() -> (i64, i64) {
    let input: Vec<Move> = util::read_input(17)[0].chars().map(|c| c.into()).collect();
    (one(&input), two(&input))
}
