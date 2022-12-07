use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

static INPUT: &str = include_str!("input");

#[derive(Debug, Default)]
struct DirState {
    sub_dirs: HashSet<String>,
    files: HashMap<String, usize>,
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(String),
}

const TARGET_SPACE: usize = 40_000_000;

impl Command {
    fn parse(s: &str) -> Self {
        if s.starts_with("$ ls") {
            return Self::Ls;
        }

        if s.starts_with("$ cd") {
            return Self::Cd(s.strip_prefix("$ cd ").unwrap().to_string());
        }

        unreachable!()
    }
}

#[derive(Debug)]
enum DirEntry {
    SubDir(String),
    File(String, usize),
}

impl DirEntry {
    fn parse(s: &str) -> Self {
        let s = s.split(" ").collect::<Vec<_>>();

        if s[0] == "dir" {
            Self::SubDir(s[1].to_string())
        } else {
            Self::File(s[1].to_string(), s[0].parse().unwrap())
        }
    }
}

fn main() {
    let mut tree = HashMap::<Vec<String>, DirState>::new();
    let mut wd = Vec::<String>::new();

    for cmd in INPUT.lines() {
        if cmd.starts_with("$") {
            if let Command::Cd(path) = Command::parse(cmd) {
                match path.as_str() {
                    "/" => {
                        wd = vec!["/".to_string()];
                    }
                    ".." => {
                        wd.pop();
                    }
                    path => {
                        wd.push(path.to_string());
                    }
                }

                tree.entry(wd.clone()).or_default();
            }
        } else {
            let dir_state = tree.get_mut(&wd).unwrap();
            match DirEntry::parse(cmd) {
                DirEntry::SubDir(sub_dir) => {
                    dir_state.sub_dirs.insert(sub_dir);
                }
                DirEntry::File(name, size) => {
                    dir_state.files.insert(name, size);
                }
            }
        }
    }

    let mut space = HashMap::<Vec<String>, usize>::new();

    fn calculate_size(
        path: Vec<String>,
        tree: &HashMap<Vec<String>, DirState>,
        space: &mut HashMap<Vec<String>, usize>,
    ) -> usize {
        let dir_state = tree.get(&path).unwrap();
        if dir_state.sub_dirs.is_empty() {
            let size = dir_state
                .files
                .iter()
                .map(|(_name, size)| size)
                .sum::<usize>();

            space.insert(path, size);

            size
        } else {
            let sub_dir_size = dir_state
                .sub_dirs
                .iter()
                .map(|sub_dir| {
                    let mut path = path.clone();
                    path.push(sub_dir.to_string());

                    calculate_size(path, tree, space)
                })
                .sum::<usize>();

            let file_size = dir_state
                .files
                .iter()
                .map(|(_name, size)| size)
                .sum::<usize>();

            let size = sub_dir_size + file_size;
            space.insert(path, size);

            size
        }
    }

    let used_space = calculate_size(vec!["/".into()], &tree, &mut space);
    let to_free = used_space - TARGET_SPACE;

    let target_dir = space
        .iter()
        .sorted_by_key(|(_path, size)| *size)
        .skip_while(|(_path, size)| **size < to_free)
        .next()
        .unwrap()
        .0;

    let result = space.get(target_dir).unwrap();

    println!("{result}");
}
