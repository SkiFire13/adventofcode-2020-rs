#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u64>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input
        .windows(25 + 1)
        .map(|window| window.split_last().unwrap())
        .find(|&(&n, last25)| {
            last25
                .iter()
                .filter(|&&i| i < n && 2 * i != n)
                .all(|&i| !last25.iter().any(|&j| j + i == n))
        })
        .map(|(&n, _)| n)
        .expect("There's no solution")
}

pub fn part2(input: &Input) -> u64 {
    let target = part1(input);

    let mut s = 0;
    let mut e = 2;
    let mut acc = input[0] + input[1];

    while s < e && e < input.len() {
        match acc.cmp(&target) {
            Ordering::Equal => {
                return input[s..e]
                    .iter()
                    .minmax()
                    .into_option()
                    .map(|(min, max)| min + max)
                    .unwrap();
            }
            Ordering::Greater => {
                acc -= input[s];
                s += 1;
            }
            Ordering::Less => {
                acc += input[e];
                e += 1;
            }
        }
        if s + 2 >= e {
            acc += input[e];
            e += 1;
        }
    }

    panic!("There's no solution");
}
