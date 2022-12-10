fn main() {
    let input = include_str!("../input");

    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut target = 20;
    let mut part_1 = 0;
    let mut part_2 = String::new();

    for line in input.lines() {
        let parts = line.split_once(' ');
        let cycles = parts.is_some() as u8 + 1;

        for _ in 0..cycles {
            let pos = (cycle - 1) % 40;
            part_2 += if (x - pos).abs() <= 1 { "#" } else { "." };

            if pos == 39 {
                part_2 += "\n";
            }

            if cycle == target {
                part_1 += x * cycle;
                target += 40;
            }

            cycle += 1;
        }

        if let Some((_, v)) = parts {
            x += v.parse::<i32>().unwrap();
        }
    }

    print!("Part 1: {part_1}\nPart 2:\n{part_2}");
}
