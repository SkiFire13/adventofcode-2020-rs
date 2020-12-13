#[allow(unused_imports)]
use super::prelude::*;
type Input = (u64, Vec<(u64, u64)>);

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse().expect("Invalid input");
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(pos, bus)| (pos as u64, bus.parse().expect("Invalid input")))
        .collect();
    (start, buses)
}

pub fn part1(input: &Input) -> u64 {
    let &(start, ref buses) = input;
    for time in start.. {
        if let Some(&bus) = buses
            .iter()
            .map(|(_, bus)| bus)
            .find(|&&bus| time % bus == 0)
        {
            return (time - start) * bus;
        }
    }
    unreachable!();
}

pub fn part2(input: &Input) -> u64 {
    let (_, buses) = input;

    let mut acc = (0, 1);
    for &(pos, bus) in buses {
        let pos = (bus - pos % bus) % bus;
        while acc.0 % bus != pos {
            acc.0 += acc.1;
        }
        acc.1 *= bus;
    }

    acc.0
}
