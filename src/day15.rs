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
    let mut history = vec![usize::MAX; nth];
    starting
        .iter()
        .copied()
        .enumerate()
        .for_each(|(pos, n)| history[n] = pos);

    let mut last = starting.last().copied().expect("Input is empty");
    let mut prev_spoken = usize::MAX;

    for i in starting.len()..nth {
        if prev_spoken == usize::MAX {
            last = 0;
        } else {
            last = i - 1 - prev_spoken;
        }
        prev_spoken = mem::replace(&mut history[last], i);
    }

    last
}

pub fn part1(input: &Input) -> usize {
    run(input, 2020)
}

pub fn part2(input: &Input) -> usize {
    run(input, 30000000)
}
