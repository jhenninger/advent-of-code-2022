fn main() {
    let input = include_str!("../input");
    let (stacks, procedure) = parse(input);

    println!(
        "Part 1: {}\nPart 2: {}",
        rearrange(&stacks, &procedure, false),
        rearrange(&stacks, &procedure, true),
    );
}

fn rearrange(stacks: &[Vec<char>], procedure: &[(usize, usize, usize)], multiple: bool) -> String {
    let mut stacks = stacks.to_vec();

    for &(count, src, dst) in procedure {
        let src = &mut stacks[src];
        let popped = src.split_off(src.len() - count);
        if multiple {
            stacks[dst].extend(popped);
        } else {
            stacks[dst].extend(popped.iter().rev());
        }
    }

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines = input.lines().peekable();
    let n = (lines.peek().unwrap().len() + 1) / 4;

    let mut stacks = vec![Vec::new(); n];

    for l in lines.by_ref() {
        if l.is_empty() {
            break;
        }

        for (i, c) in l.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse()
    }

    let procedure: Vec<_> = lines
        .map(|l| {
            let mut iter = l
                .split_ascii_whitespace()
                .skip(1)
                .step_by(2)
                .map(|n| n.parse().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        })
        .collect();

    (stacks, procedure)
}
