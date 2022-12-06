use std::collections::HashSet;

fn main() {
    let input = include_bytes!("../input");

    println!(
        "Part 1: {}\nPart 2: {}",
        start_of_message(input, 4).unwrap(),
        start_of_message(input, 14).unwrap(),
    );
}

fn start_of_message(buffer: &[u8], len: usize) -> Option<usize> {
    buffer
        .windows(len)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == len)
        .map(|n| n + len)
}
