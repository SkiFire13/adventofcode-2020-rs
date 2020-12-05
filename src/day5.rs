#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u16>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            line
                .chars()
                .fold(0, |acc, c| (acc << 1) + (c == 'B' || c == 'R') as u16)
        })
        .sorted()
        .collect()
}

pub fn part1(input: &Input) -> u16 {
    let seats = input;
    seats
        .iter()
        .next_back()
        .copied()
        .expect("Invalid input")
}

pub fn part2(input: &Input) -> u16 {
    let seats = input;
    seats.iter()
        .zip(seats.iter().skip(1))
        .find(|&(&id1, &id2)| id2 != id1 + 1)
        .map(|(&id1, _)| id1 + 1)
        .expect("No matching id found")
}
