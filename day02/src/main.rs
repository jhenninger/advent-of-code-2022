fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}\nPart 2: {}", part_1(&input), part_2(&input))
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3 + 1,
            "B Y" => 3 + 2,
            "C Z" => 3 + 3,
            "A Y" => 6 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "C Y" => 0 + 2,
            _ => panic!(),
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A Y" => 3 + 1,
            "B Y" => 3 + 2,
            "C Y" => 3 + 3,
            "A X" => 0 + 3,
            "B X" => 0 + 1,
            "C X" => 0 + 2,
            "A Z" => 6 + 2,
            "B Z" => 6 + 3,
            "C Z" => 6 + 1,
            _ => panic!(),
        })
        .sum()
}
