use std::collections::{HashMap, HashSet};

use regex::Regex;

use aoc2022::util::{read_input, ToInputVec};

#[derive(Debug, Clone)]
struct Valve {
    rate: i32,
    tunnels: Vec<String>,
}

fn get_shortest_paths(valves: &HashMap<String, Valve>) -> HashMap<String, Vec<(String, i32)>> {
    let mut distances: HashMap<(String, String), i32> = HashMap::new();

    for (name, Valve { tunnels, .. }) in valves {
        for tunnel in tunnels {
            distances.insert((name.clone(), name.clone()), 0);
            distances.insert((name.clone(), tunnel.clone()), 1);
        }
    }

    for name in valves.keys() {
        let mut distance = 1;
        let mut visited = distances
            .iter()
            .filter_map(|((from, to), _)| if from == name { Some(to.clone()) } else { None })
            .collect::<Vec<_>>();
        let mut previous_valves = visited.clone();

        loop {
            let visited_clone = visited.clone();
            let previous_valves_clone = previous_valves.clone();

            let next_valves = valves
                .iter()
                .filter_map(|(name, valve)| {
                    if previous_valves_clone.contains(name) {
                        Some(valve.tunnels.clone())
                    } else {
                        None
                    }
                })
                .flatten()
                .filter(|name| !visited_clone.contains(name));

            previous_valves = next_valves.clone().collect();

            if next_valves.clone().peekable().peek().is_none() {
                break;
            }

            distance += 1;
            visited.extend(next_valves.clone());
            distances.extend(next_valves.map(|next_name| ((name.clone(), next_name.clone()), distance)));
        }
    }

    let mut shortest_paths: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    for ((from, to), distance) in distances.into_iter().filter(|((from, to), _)| from != to) {
        shortest_paths
            .entry(from)
            .and_modify(|paths| paths.push((to.clone(), distance)))
            .or_insert(vec![(to.clone(), distance)]);
    }

    shortest_paths
}

fn maximize_pressure(
    valves: &HashMap<String, Valve>,
    shortest_paths: &HashMap<String, Vec<(String, i32)>>,
    opened: HashSet<String>,
    current: &String,
    time_left: i32,
) -> i32 {
    if time_left <= 0 {
        return 0;
    }

    let current_valve = valves.get(current).unwrap();
    let mut maximum = (time_left - 1) * current_valve.rate;

    for (to, path_len) in shortest_paths.get(current).unwrap() {
        if opened.contains(to) || valves.get(to).unwrap().rate == 0 {
            continue;
        }

        let time_left_after = time_left - path_len - current_valve.rate.signum().abs();
        let mut opened_current = opened.clone();
        opened_current.insert(current.clone());

        maximum = maximum.max(
            maximize_pressure(valves, shortest_paths, opened_current, &to.clone(), time_left_after)
                + (time_left - 1) * current_valve.rate,
        );
    }

    maximum
}

fn maximize_pressure_elephant(
    valves: &HashMap<String, Valve>,
    shortest_paths: &HashMap<String, Vec<(String, i32)>>,
    opened: HashSet<String>,
    current_human: &String,
    current_elephant: &String,
    time_left_human: i32,
    time_left_elephant: i32,
) -> i32 {
    if time_left_human.max(time_left_elephant) <= 0 {
        return 0;
    }

    let current_human_valve = valves.get(current_human).unwrap();
    let current_elephant_valve = valves.get(current_elephant).unwrap();

    let human_rate = current_human_valve.rate;

    let elephant_rate = if current_human == current_elephant {
        0
    } else {
        current_elephant_valve.rate
    };

    let human_pressure = if time_left_human <= 0 {
        0
    } else {
        (time_left_human - 1) * human_rate
    };

    let elephant_pressure = if time_left_elephant <= 0 {
        0
    } else {
        (time_left_elephant - 1) * elephant_rate
    };

    let mut maximum = human_pressure + elephant_pressure;

    for (to_human, path_len_human) in shortest_paths.get(current_human).unwrap() {
        for (to_elephant, path_len_elephant) in shortest_paths.get(current_elephant).unwrap() {
            if opened.contains(to_human) || valves.get(to_human).unwrap().rate == 0 {
                continue;
            } else if opened.contains(to_elephant) || valves.get(to_elephant).unwrap().rate == 0 {
                continue;
            }

            let mut opened_current = opened.clone();
            opened_current.insert(current_human.clone());
            opened_current.insert(current_elephant.clone());

            let time_left_after_human = time_left_human - path_len_human - human_rate.signum().abs();
            let time_left_after_elephant = time_left_elephant - path_len_elephant - elephant_rate.signum().abs();

            maximum = maximum.max(
                maximize_pressure_elephant(
                    valves,
                    shortest_paths,
                    opened_current,
                    &to_human.clone(),
                    &to_elephant.clone(),
                    time_left_after_human,
                    time_left_after_elephant,
                ) + human_pressure
                    + elephant_pressure,
            );
        }
    }

    maximum
}

fn problem_one(input: &String) -> i32 {
    let input = parse_input(input);
    let shortest_paths = get_shortest_paths(&input);

    maximize_pressure(&input, &shortest_paths, HashSet::new(), &"AA".to_string(), 30)
}

fn problem_two(input: &String) -> i32 {
    let input = parse_input(input);
    let shortest_paths = get_shortest_paths(&input);

    maximize_pressure_elephant(
        &input,
        &shortest_paths,
        HashSet::new(),
        &"AA".to_string(),
        &"AA".to_string(),
        26,
        26,
    )
}

fn main() {
    let input = read_input(16);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

fn parse_input(input: &String) -> HashMap<String, Valve> {
    let re = Regex::new(r"Valve (.+?) .* rate=(\d+);.*valves? (.+(, )?)+").unwrap();

    input
        .to_vec()
        .iter()
        .map(|s| {
            let captures = re.captures(s).unwrap();

            (
                captures.get(1).unwrap().as_str().to_string(),
                Valve {
                    rate: captures.get(2).unwrap().as_str().parse().unwrap(),
                    tunnels: captures
                        .get(3)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|s| s.to_string())
                        .collect(),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 1651);
    }

    #[test]
    #[ignore]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.trim().to_string()), 1707);
    }
}
