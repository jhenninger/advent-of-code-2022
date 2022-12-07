use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let mut cwd = Vec::new();
    let mut dirs = HashMap::new();

    for line in input.lines() {
        let words: Vec<_> = line.split_ascii_whitespace().collect();

        match &words[..] {
            ["$", "cd", ".."] => {
                let dir = *dirs.get_mut(&cwd).unwrap();
                cwd.pop();
                *dirs.get_mut(&cwd).unwrap() += dir;
            }
            ["$", "cd", dir] => {
                cwd.push(*dir);
                dirs.insert(cwd.clone(), 0);
            }
            ["dir" | "ls" | "$", ..] => (), // noop
            [size, _] => {
                let dir = dirs.get_mut(&cwd).unwrap();
                *dir += size.parse::<usize>().unwrap()
            }
            _ => panic!()
        }
    }

    let part_1: usize = dirs.values().filter(|v| **v <= 100000).sum();

    let mut sizes: Vec<usize> = dirs.values().cloned().collect();
    sizes.sort_unstable();

    let target = 30000000 - (70000000 - sizes.last().unwrap());
    let part_2 = sizes.iter().find(|v| **v >= target).unwrap();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
