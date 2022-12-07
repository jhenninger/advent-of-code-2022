use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let mut cwd = Vec::new();
    let mut dirs = HashMap::new();

    for line in input.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();

        match &words[..] {
            ["$", "cd", ".."] => {
                cwd.pop();
            }
            ["$", "cd", dir] => {
                cwd.push(*dir);
                dirs.insert(cwd.clone(), 0);
            }
            ["$" | "dir", ..] => (), // noop
            [size, _] => {
                let size: usize = size.parse().unwrap();
                for i in 1..=cwd.len() {
                    *dirs.get_mut(&cwd[0..i]).unwrap() += size;
                }
            }
            _ => panic!(),
        }
    }

    let part_1: usize = dirs.values().filter(|v| **v <= 100000).sum();

    let target = 30000000 - (70000000 - dirs.values().max().unwrap());
    let part_2 = dirs.values().filter(|v| **v >= target).min().unwrap();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
