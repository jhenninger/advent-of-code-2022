use std::cmp::Ordering;

use Packet::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Number(i32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Number(a), Number(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Number(_), List(b)) => [self.clone()].as_slice().cmp(b),
            (List(a), Number(_)) => a.as_slice().cmp(&[other.clone()]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn from_str(s: &str) -> Packet {
        fn inner(s: &mut &str) -> Packet {
            let mut vec = Vec::new();

            while !s.is_empty() {
                match &s[0..1] {
                    "[" => {
                        *s = &s[1..];
                        vec.push(inner(s));
                    }
                    "]" => {
                        *s = &s[1..];
                        return List(vec);
                    }
                    "," => *s = &s[1..],
                    _ => {
                        let idx = s.find(|c: char| !c.is_digit(10)).unwrap();
                        vec.push(Number(s[..idx].parse().unwrap()));
                        *s = &s[idx..];
                    }
                }
            }

            List(vec)
        }

        inner(&mut &s[1..s.len()])
    }
}

fn main() {
    let input = include_str!("../input");

    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from_str(l))
        .collect();

    let part_1: usize = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, c)| c[0] < c[1])
        .map(|(n, _)| n + 1)
        .sum();

    let dividers = [Packet::from_str("[[2]]"), Packet::from_str("[[6]]")];

    packets.extend(dividers.iter().cloned());
    packets.sort_unstable();

    let part_2: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(n, _)| n + 1)
        .product();

    println!("Part 1: {part_1}\nPart 2: {part_2}");
}
