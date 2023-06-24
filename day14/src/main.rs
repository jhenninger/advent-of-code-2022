use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Clone)]
struct Cave {
    set: HashSet<(i32, i32)>,
    max_y: i32,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut set = HashSet::new();

        for line in input.lines() {
            let stops = line.split(" -> ").map(|line| {
                let (x, y) = line.split_once(',').unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            });

            for ((x1, y1), (x2, y2)) in stops.clone().zip(stops.skip(1)) {
                for x in min(x1, x2)..=max(x1, x2) {
                    for y in min(y1, y2)..=max(y1, y2) {
                        set.insert((x, y));
                    }
                }
            }
        }

        Self {
            max_y: set.iter().map(|k| k.1).max().unwrap(),
            set,
        }
    }

    fn part_1(&self) -> usize {
        let mut set = self.set.clone();
        let mut c = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                if y > self.max_y {
                    return c;
                }

                if !set.contains(&(x, y + 1)) {
                    y += 1;
                    continue;
                }

                if !set.contains(&(x - 1, y + 1)) {
                    x -= 1;
                    y += 1;
                    continue;
                }

                if !set.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                    continue;
                }

                set.insert((x, y));

                break;
            }

            c += 1;
        }
    }

    fn part_2(&self) -> usize {
        let mut set = self.set.clone();
        let mut c = 0;

        loop {
            let mut x = 500;
            let mut y = 0;

            c += 1;

            loop {
                if y == self.max_y + 1 {
                    set.insert((x, y));
                    break;
                }

                if !set.contains(&(x, y + 1)) {
                    y += 1;
                    continue;
                }

                if !set.contains(&(x - 1, y + 1)) {
                    x -= 1;
                    y += 1;
                    continue;
                }

                if !set.contains(&(x + 1, y + 1)) {
                    x += 1;
                    y += 1;
                    continue;
                }

                if x == 500 && y == 0 {
                    return c;
                }

                set.insert((x, y));
                break;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let cave = Cave::new(input);

    println!("Part 1: {}\nPart 2: {}", cave.part_1(), cave.part_2());
}