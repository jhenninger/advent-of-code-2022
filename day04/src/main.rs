fn main() {
    let input = include_str!("../input");

    let ranges: Vec<((u32, u32), (u32, u32))> = input
        .lines()
        .map(|l| {
            let mut parts = l.split(['-', ',']).map(|n| n.parse().unwrap());
            (
                (parts.next().unwrap(), parts.next().unwrap()),
                (parts.next().unwrap(), parts.next().unwrap()),
            )
        })
        .collect();

    let part_1 = ranges
        .iter()
        .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
        .count();

    let part_2 = ranges
        .iter()
        .filter(|(a, b)| a.1 >= b.0 && b.1 >= a.0)
        .count();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
