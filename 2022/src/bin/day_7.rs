use std::collections::HashMap;

use regex::Regex;

use aoc2022::util::{read_input, ToInputVec};

#[derive(Debug, Clone)]
enum FileSystemItem {
    File(i32),
    Dir(String),
}

#[derive(Debug)]
enum CommandOutput {
    Cd(String),
    Ls(Vec<FileSystemItem>),
}

#[derive(Debug, Clone)]
struct Directory {
    files: Vec<FileSystemItem>,
}

impl Directory {
    fn empty() -> Directory {
        Directory { files: vec![] }
    }

    fn add_file(&mut self, file: FileSystemItem) {
        self.files.push(file);
    }
}

fn parse_command(cmd: &String, output: Vec<&String>) -> CommandOutput {
    let cd: Regex = Regex::new(r"\$ cd (.+)").unwrap();
    let ls: Regex = Regex::new(r"\$ ls").unwrap();
    let dir: Regex = Regex::new(r"dir (.+)").unwrap();
    let file: Regex = Regex::new(r"(\d+) (.+)").unwrap();

    if ls.is_match(cmd) {
        CommandOutput::Ls(
            output
                .iter()
                .map(|line| {
                    if let Some(captures) = dir.captures(line) {
                        FileSystemItem::Dir(captures.get(1).unwrap().as_str().to_string())
                    } else if let Some(captures) = file.captures(line) {
                        FileSystemItem::File(captures.get(1).unwrap().as_str().parse::<i32>().unwrap())
                    } else {
                        panic!()
                    }
                })
                .collect(),
        )
    } else if let Some(captures) = cd.captures(cmd) {
        CommandOutput::Cd(captures.get(1).unwrap().as_str().to_string())
    } else {
        panic!()
    }
}

fn to_path(dir: &Vec<String>) -> String {
    if dir.len() > 1 {
        format!("/{}", dir[1..].join("/"))
    } else {
        "/".to_string()
    }
}

fn get_directory_sizes(input: &Vec<String>) -> HashMap<String, i32> {
    let mut it = input.iter().peekable();
    let mut current_dir: Vec<String> = vec![];
    let mut directories: HashMap<String, Directory> = HashMap::new();

    loop {
        let current_path = to_path(&current_dir);

        if !directories.contains_key(&current_path) && !current_dir.is_empty() {
            directories.insert(current_path.clone(), Directory::empty());
        }

        if let Some(cmd) = it.next() {
            let mut buf = vec![];

            while let Some(line) = it.next_if(|line| !line.starts_with('$')) {
                buf.push(line);
            }

            let cmd = parse_command(cmd, buf);

            if let CommandOutput::Cd(name) = cmd {
                if name == ".." {
                    current_dir.pop();
                } else {
                    current_dir.push(name);
                }
            } else if let CommandOutput::Ls(listing) = cmd {
                for item in listing {
                    if let file @ FileSystemItem::File(_) = item {
                        directories.get_mut(&current_path).unwrap().add_file(file)
                    }
                }
            }
        } else {
            break;
        }
    }

    let mut sizes: HashMap<String, i32> = HashMap::new();

    for (k, v) in directories {
        let size: i32 = v
            .files
            .iter()
            .filter_map(|item| match item {
                FileSystemItem::File(size) => Some(size),
                _ => None,
            })
            .sum();

        let parents = k.split("/").collect::<Vec<_>>();

        for i in 0..parents.len() {
            let parent = parents[0..=i].join("/").to_string();
            let size = sizes.get(&parent).unwrap_or(&0) + size;
            sizes.insert(parent, size);
        }
    }

    sizes
}

fn problem_one(input: &String) -> i32 {
    get_directory_sizes(&input.to_vec())
        .values()
        .filter(|&&size| size < 100000)
        .map(|&size| size)
        .sum::<i32>()
}

fn problem_two(input: &String) -> i32 {
    let sizes = get_directory_sizes(&input.to_vec());
    let root_size = *sizes.get("").unwrap();
    let space_to_free = root_size - 40000000;
    let mut smallest = root_size;

    for (key, size) in sizes {
        if key != "/" {
            if size > space_to_free && size < smallest {
                smallest = size;
            }
        }
    }

    smallest.clone()
}

fn main() {
    let input = read_input(7);
    println!("Problem one: {}", problem_one(&input));
    println!("Problem two: {}", problem_two(&input));
}

mod tests {
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_problem_one() {
        assert_eq!(super::problem_one(&TEST_INPUT.to_string()), 95437);
    }

    #[test]
    fn test_problem_two() {
        assert_eq!(super::problem_two(&TEST_INPUT.to_string()), 24933642);
    }
}
