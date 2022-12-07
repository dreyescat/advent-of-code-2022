use std::collections::HashMap;
use std::{
    io,
    path::{Path, PathBuf},
};

const DISK_SPACE: u32 = 70_000_000;
const UPDATE_SPACE: u32 = 30_000_000;

fn main() {
    let mut filesystem = HashMap::new();
    let mut path = PathBuf::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let command: Vec<&str> = line.split_ascii_whitespace().collect();
        match &command[..] {
            ["$", "cd", "/"] => {
                path.push("/");
            }
            ["$", "cd", ".."] => {
                path.pop();
            }
            ["$", "cd", directory] => {
                path.push(directory);
            }
            ["$", "ls"] => (),
            ["dir", _d] => (),
            [size, _file] => {
                let size: u32 = size.parse().unwrap();
                update_sizes(&mut filesystem, &path, size);
            }
            _ => panic!("command not found: {}", command.join(" ")),
        }
    }

    let total_size: u32 = filesystem.values().filter(|&value| *value <= 100000).sum();
    println!("{}", total_size);

    let required_space = UPDATE_SPACE - (DISK_SPACE - filesystem.get("/").unwrap());
    let directory_size = filesystem
        .values()
        .filter(|&value| *value > required_space)
        .min();
    println!("{}", directory_size.unwrap());
}

fn update_sizes(filesystem: &mut HashMap<String, u32>, path: &Path, size: u32) {
    for p in path.ancestors() {
        filesystem
            .entry(p.display().to_string())
            .and_modify(|s| *s += size)
            .or_insert(size);
    }
}
