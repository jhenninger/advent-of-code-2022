use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let motions: Vec<((i8, i8), u32)> = input
        .lines()
        .map(|l| {
            let (dir, steps) = l.split_once(" ").unwrap();
            let steps = steps.parse().unwrap();

            let dir = match dir {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => panic!(),
            };

            (dir, steps)
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        solve::<2>(&motions),
        solve::<10>(&motions),
    );
}

fn solve<const N: usize>(motions: &[((i8, i8), u32)]) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [(0i32, 0i32); N];

    for &(dir, steps) in motions {
        for _ in 0..steps {
            rope[0].0 += dir.0 as i32;
            rope[0].1 += dir.1 as i32;

            for i in 1..N {
                let head = rope[i - 1];
                let tail = &mut rope[i];
                let dist = (head.0 - tail.0, head.1 - tail.1);

                if dist.0.abs() < 2 && dist.1.abs() < 2 {
                    break;
                }

                tail.0 += dist.0.signum();
                tail.1 += dist.1.signum();
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}
