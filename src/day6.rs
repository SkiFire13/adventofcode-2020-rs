#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = Vec<Vec<&'a str>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|group| group.lines().collect())
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .flat_map(|s| s.chars())
                .fold(0u32, |acc, c| acc | (1 << (c as u8 - b'a')))
                .count_ones()
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .fold((1 << 26) - 1, |acc, chars| {
                    chars.chars().fold(0u32, |inner_acc, c| {
                        inner_acc | (acc & (1 << (c as u8 - b'a')))
                    })
                })
                .count_ones()
        })
        .sum()
}
