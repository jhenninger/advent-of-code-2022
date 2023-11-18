fn main() {
    let input = include_str!("../input").trim();
    let mut numbers: Vec<(usize, i64)> = input
        .lines()
        .enumerate()
        .map(|(i, n)| (i, n.parse().unwrap()))
        .collect();

    let len = numbers.len();

    let part_1 = decrypt(numbers.clone(), len);

    for e in numbers.iter_mut() {
        e.1 *= 811589153
    }

    let part_2 = decrypt(numbers, len * 10);

    println!("Part 1: {part_1}\nPart 2: {part_2}");

}

fn decrypt(mut numbers: Vec<(usize, i64)>, rounds: usize) -> i64 {
    for i in 0..rounds {
        let pos = numbers
            .iter()
            .position(|(pos, _)| *pos == i % numbers.len())
            .unwrap();

        let e = numbers.remove(pos);
        let new_pos = (pos as i64 + e.1).rem_euclid(numbers.len() as i64) as usize;

        numbers.insert(new_pos, e);
    }

    let pos = numbers.iter().position(|(_, n)| *n == 0).unwrap();

    numbers[(pos + 1000) % numbers.len()].1
        + numbers[(pos + 2000) % numbers.len()].1
        + numbers[(pos + 3000) % numbers.len()].1
}
