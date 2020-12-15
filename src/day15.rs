#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<usize>;

pub fn input_generator(input: &str) -> Input {
    input
        .split(',')
        .map(|n| n.parse().expect("Invalid input"))
        .collect()
}

fn run(starting: &[usize], nth: usize) -> usize {
    let mut history = starting
        .iter()
        .copied()
        .enumerate()
        .map(|(pos, n)| (n, pos))
        .collect::<HashMap<_, _>>();
    history.reserve(nth / 5);
    let mut last = starting.last().copied().expect("Input is empty");
    let mut prev_spoken = usize::MAX;

    for i in starting.len()..nth {
        if prev_spoken == usize::MAX {
            last = 0;
        } else {
            last = i - 1 - prev_spoken;
        }
        prev_spoken = history.insert(last, i).unwrap_or(usize::MAX);
    }

    last
}

pub fn part1(input: &Input) -> usize {
    run(input, 2020)
}

pub fn part2(input: &Input) -> usize {
    run(input, 30000000)
}
