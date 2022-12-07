use std::collections::HashMap;
use std::{
    io,
    path::{Path, PathBuf},
};

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
}

fn update_sizes(filesystem: &mut HashMap<String, u32>, path: &Path, size: u32) {
    for p in path.ancestors() {
        filesystem
            .entry(p.display().to_string())
            .and_modify(|s| *s += size)
            .or_insert(size);
    }
}
