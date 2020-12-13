#[allow(unused_imports)]
use super::prelude::*;
type Input = (i64, Vec<(i64, i64)>);

pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let start = lines.next().unwrap().parse().expect("Invalid input");
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, bus)| bus != "x")
        .map(|(pos, bus)| (pos as i64, bus.parse().expect("Invalid input")))
        .collect();
    (start, buses)
}

pub fn part1(input: &Input) -> i64 {
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

pub fn part2(input: &Input) -> i64 {
    let (_, buses) = input;

    let mut acc = (0, 1);
    for &(pos, bus) in buses {
        let (a1, a2) = acc;
        let (b1, b2) = ((bus - pos % bus) % bus, bus);

        let (mut r1, mut r2) = (a2, b2);
        let (mut s1, mut s2) = (1, 0);
        let (mut t1, mut t2) = (0, 1);
        while r2 != 0 {
            let q = r1 / r2;
            let r3 = r1 - q * r2;
            r1 = r2;
            r2 = r3;
            let s3 = s1 - q * s2;
            s1 = s2;
            s2 = s3;
            let t3 = t1 - q * t2;
            t1 = t2;
            t2 = t3;
        }
        let s1 = (s1 / r1) as i128;
        let t1 = (t1 / r1) as i128;

        let (a1, a2) = (a1 as i128, a2 as i128);
        let (b1, b2) = (b1 as i128, b2 as i128);

        let acc0 = b1 * s1 * a2 + a1 * t1 * b2;
        let acc1 = a2 * b2;

        acc = (acc0.rem_euclid(acc1) as i64, acc1 as i64);
    }

    acc.0
}
