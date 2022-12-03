use std::collections::HashSet;

fn main() {
    let rucksacks: Vec<_> = include_str!("../input").lines().collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        part_1(&rucksacks),
        part_2(&rucksacks)
    );
}

fn part_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            let left: HashSet<_> = left.chars().collect();
            let right: HashSet<_> = right.chars().collect();
            let common = *left.intersection(&right).next().unwrap();

            priority(common) as usize
        })
        .sum()
}

fn part_2(input: &[&str]) -> usize {
    input
        .chunks(3)
        .map(|c| {
            let common = *c
                .iter()
                .map(|l| l.chars().collect::<HashSet<_>>())
                .reduce(|acc, e| acc.intersection(&e).cloned().collect())
                .unwrap()
                .iter()
                .next()
                .unwrap();

            priority(common) as usize
        })
        .sum()
}

fn priority(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        c as u8 - b'a' + 1
    } else {
        c as u8 - b'A' + 27
    }
}
