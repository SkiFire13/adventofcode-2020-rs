#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u32>;

pub fn input_generator(input: &str) -> Input {
    let mut v = input.lines()
        .map(|line| line.parse().expect("Invalid input"))
        .collect::<Vec<_>>();
    v.sort_unstable();
    v
}

fn find_sum_2(numbers: &[u32], target: u32) -> Option<u32> {
    if numbers.len() < 2 || numbers[0] + numbers[1] > 2020 { return None; }
    let mut lo = 0;
    let mut hi = numbers.len() - 1;
    while lo != hi {
        match (numbers[lo] + numbers[hi]).cmp(&target) {
            Ordering::Equal => return Some(numbers[lo] * numbers[hi]),
            Ordering::Greater => hi -= 1,
            Ordering::Less => lo += 1,
        }
    }
    None
} 

pub fn part1(input: &Input) -> u32 {
    let numbers = input;
    find_sum_2(&numbers, 2020).expect("There's no solution")
}

pub fn part2(input: &Input) -> u32 {
    let numbers = input;
    for (i, n) in numbers.iter().copied().enumerate() {
        if n > 2020 { break; }
        if let Some(prod) = find_sum_2(&numbers[i+1..], 2020 - n) {
            return n * prod;
        }
    }
    panic!("There's no solution")
}
