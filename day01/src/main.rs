fn main() {
    let input = include_str!("../input");

    let mut calories: Vec<usize> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|l| l.parse::<usize>().unwrap()).sum())
        .collect();

    calories.sort_unstable();

    let part_1 = calories.last().unwrap();
    let part_2: usize = calories.iter().rev().take(3).sum();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
