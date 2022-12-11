use regex::Regex;
use std::cmp::Reverse;
use std::ops::{Add, Mul};

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    modulo: u64,
    operator: fn(u64, u64) -> u64,
    operand: Operand,
    r#true: usize,
    r#false: usize,
    inspections: u64,
}

impl Monkey {
    fn throw(&mut self, modulo: Option<u64>) -> Option<(usize, u64)> {
        let mut i = self.items.pop()?;

        let rhs = match self.operand {
            Operand::Constant(c) => c,
            Operand::Old => i,
        };

        i = (self.operator)(i, rhs);

        match modulo {
            Some(modulo) => i %= modulo,
            None => i /= 3,
        }

        let target = match i % self.modulo {
            0 => self.r#true,
            _ => self.r#false,
        };

        self.inspections += 1;

        Some((target, i))
    }
}

#[derive(Clone, Debug)]
enum Operand {
    Constant(u64),
    Old,
}

fn main() {
    let input = include_str!("../input");
    let regex = Regex::new(
        r"
Monkey .+:
  Starting items: (.+)
  Operation: new = old (.) (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"
            .trim(),
    )
    .unwrap();

    let monkeys: Vec<_> = regex
        .captures_iter(input)
        .map(|c| {
            let operator = match &c[2] {
                "+" => u64::add,
                "*" => u64::mul,
                _ => panic!(),
            };

            let operand = match &c[3] {
                "old" => Operand::Old,
                n => Operand::Constant(n.parse().unwrap()),
            };

            Monkey {
                items: c[1].split(", ").map(|i| i.parse().unwrap()).collect(),
                operator,
                operand,
                modulo: c[4].parse().unwrap(),
                r#true: c[5].parse().unwrap(),
                r#false: c[6].parse().unwrap(),
                inspections: 0,
            }
        })
        .collect();

    println!(
        "Part 1: {}\nPart 2: {}",
        monkey_business(monkeys.clone(), false),
        monkey_business(monkeys, true)
    );
}

fn monkey_business(mut monkeys: Vec<Monkey>, part_2: bool) -> u64 {
    let (rounds, modulo) = if part_2 {
        let modulo = monkeys.iter().map(|m| m.modulo).product();
        (10000, Some(modulo))
    } else {
        (20, None)
    };

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while let Some((target, item)) = monkeys[m].throw(modulo) {
                monkeys[target].items.push(item);
            }
        }
    }

    monkeys.sort_unstable_by_key(|m| Reverse(m.inspections));
    monkeys[0].inspections * monkeys[1].inspections
}
