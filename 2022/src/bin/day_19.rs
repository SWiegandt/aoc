use aoc2022::util::{parse_capture, read_input};
use regex::Regex;
use std::{collections::HashMap, ops::Mul};

fn div_ceil(lhs: i32, rhs: i32) -> i32 {
    if lhs == 0 {
        return 0;
    }

    let d = lhs / rhs;
    let r = lhs % rhs;

    if (r > 0 && rhs > 0) || (r < 0 && rhs < 0) {
        d + 1
    } else {
        d
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn collect(&self) -> Resources {
        match self {
            Robot::Ore => Resources {
                ore: 1,
                ..Default::default()
            },
            Robot::Clay => Resources {
                clay: 1,
                ..Default::default()
            },
            Robot::Obsidian => Resources {
                obsidian: 1,
                ..Default::default()
            },
            Robot::Geode => Resources {
                geode: 1,
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint(HashMap<Robot, Resources>, i32);

impl Blueprint {
    fn next_robot_build(&self, resources: &Resources, robots: &Vec<Robot>, robot: &Robot) -> Option<(i32, Resources)> {
        let robot_cost = self.0.get(robot).unwrap().clone();

        (if resources.can_pay(&robot_cost) {
            Some(0)
        } else if robot_cost.clay > 0 && !robots.contains(&Robot::Clay) {
            None
        } else if robot_cost.obsidian > 0 && !robots.contains(&Robot::Obsidian) {
            None
        } else {
            Some(
                div_ceil(
                    robot_cost.ore - resources.ore,
                    robots.iter().filter(|&r| r == &Robot::Ore).count() as i32,
                )
                .max(div_ceil(
                    robot_cost.clay - resources.clay,
                    robots.iter().filter(|&r| r == &Robot::Clay).count() as i32,
                ))
                .max(div_ceil(
                    robot_cost.obsidian - resources.obsidian,
                    robots.iter().filter(|&r| r == &Robot::Obsidian).count() as i32,
                )),
            )
        })
        .map(|n| (n, robot_cost))
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Resources {
    fn can_pay(&self, other: &Resources) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian && self.geode >= other.geode
    }

    fn add(&mut self, other: &Resources) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }

    fn subtract(&mut self, other: &Resources) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }

    fn from_robot(&self, robot: &Robot) -> i32 {
        match robot {
            Robot::Ore => self.ore,
            Robot::Clay => self.clay,
            Robot::Obsidian => self.obsidian,
            Robot::Geode => self.geode,
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
        }
    }
}

impl Mul<i32> for Resources {
    type Output = Resources;

    fn mul(self, rhs: i32) -> Self::Output {
        Resources {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

fn parse_input(input: &String) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d+):.*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*?(\d+).*").unwrap();

    input
        .lines()
        .map(|row| {
            let captures = re.captures(row).unwrap();

            Blueprint(
                HashMap::from_iter(vec![
                    (
                        Robot::Ore,
                        Resources {
                            ore: parse_capture(&captures, 2),
                            ..Default::default()
                        },
                    ),
                    (
                        Robot::Clay,
                        Resources {
                            ore: parse_capture(&captures, 3),
                            ..Default::default()
                        },
                    ),
                    (
                        Robot::Obsidian,
                        Resources {
                            ore: parse_capture(&captures, 4),
                            clay: parse_capture(&captures, 5),
                            ..Default::default()
                        },
                    ),
                    (
                        Robot::Geode,
                        Resources {
                            ore: parse_capture(&captures, 6),
                            obsidian: parse_capture(&captures, 7),
                            ..Default::default()
                        },
                    ),
                ]),
                parse_capture(&captures, 1),
            )
        })
        .collect()
}

fn get_max_geodes(blueprint: &Blueprint, robots: Vec<Robot>, mut resources: Resources, start: i32, stop: i32) -> i32 {
    let mut max_geode = 0;

    let time_until_geode = blueprint
        .next_robot_build(&resources, &robots, &Robot::Geode)
        .map(|(n, _)| n);

    for robot in [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode] {
        if robots.iter().filter(|&r| r == &robot).count() as i32
            >= blueprint.0.values().map(|r| r.from_robot(&robot)).max().unwrap()
            && robot != Robot::Geode
        {
            continue;
        }

        if let Some((time_until_build, robot_cost)) = blueprint.next_robot_build(&resources, &robots, &robot) {
            if time_until_build >= stop - start
                || (time_until_geode.map(|n| n <= time_until_build).unwrap_or(false) && robot != Robot::Geode)
            {
                continue;
            }

            let mut robots = robots.clone();
            let mut resources = resources.clone();

            for robot in robots.iter() {
                resources.add(&(robot.collect() * (time_until_build + 1)));
            }

            resources.subtract(&robot_cost);
            robots.push(robot.clone());

            max_geode = max_geode.max(get_max_geodes(
                blueprint,
                robots,
                resources,
                start + time_until_build + 1,
                stop,
            ));
        }
    }

    if stop >= start {
        for robot in robots.iter() {
            resources.add(&(robot.collect() * (stop - start + 1)));
        }
    }

    max_geode.max(resources.geode)
}

fn problem_one(input: &String) -> i32 {
    let blueprints = parse_input(input).to_vec();

    blueprints
        .iter()
        .map(|bp| bp.1 * get_max_geodes(bp, vec![Robot::Ore], Default::default(), 1, 24))
        .sum()
}

fn problem_two(input: &String) -> i32 {
    let blueprints = parse_input(input)[..3].to_vec();

    blueprints
        .iter()
        .map(|bp| get_max_geodes(bp, vec![Robot::Ore], Default::default(), 1, 32))
        .product()
}

fn main() {
    let input = read_input(19);
    println!("Problem one: {:?}", problem_one(&input));
    println!("Problem two: {:?}", problem_two(&input));
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.trim().to_string()), 33);
    }
}
