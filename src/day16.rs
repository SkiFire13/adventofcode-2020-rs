#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Vec<(&'a str, u64, u64, u64, u64)>, Vec<u64>, Vec<Vec<u64>>);

pub fn input_generator(input: &str) -> Input {
    let (rules, rest) = input
        .split("\n\nyour ticket:\n")
        .collect_tuple()
        .expect("Invalid input");
    let (myticket, nearbytickets) = rest
        .split("\n\nnearby tickets:\n")
        .collect_tuple()
        .expect("Invalid input");

    let rules = rules
        .lines()
        .map(|line| {
            let (name, rest) = line.split(": ").collect_tuple()?;
            let (r1, r2) = rest.split(" or ").collect_tuple()?;
            let (r1s, r1e) = r1.split('-').collect_tuple()?;
            let (r2s, r2e) = r2.split('-').collect_tuple()?;
            Some((
                name,
                r1s.parse().ok()?,
                r1e.parse().ok()?,
                r2s.parse().ok()?,
                r2e.parse().ok()?,
            ))
        })
        .collect::<Option<_>>()
        .expect("Invalid input");
    let myticket = myticket
        .split(',')
        .map(|v| v.parse().expect("Invalid input"))
        .collect();
    let nearbytickets = nearbytickets
        .lines()
        .map(|line| {
            line.split(',')
                .map(|v| v.parse().expect("Invalid input"))
                .collect()
        })
        .collect();

    (rules, myticket, nearbytickets)
}

pub fn part1(input: &Input) -> u64 {
    let (rules, _, nearbytickets) = input;
    let mut error_rate = 0;

    for nearbyticket in nearbytickets {
        for &v in nearbyticket {
            if !rules
                .iter()
                .any(|&(_, r1s, r1e, r2s, r2e)| (r1s <= v && v <= r1e) || (r2s <= v && v <= r2e))
            {
                error_rate += v;
            }
        }
    }

    error_rate
}

pub fn part2(input: &Input) -> u64 {
    let (rules, myticket, nearbytickets) = input;
    let mut nearbytickets = nearbytickets.clone();

    nearbytickets.retain(|nearbyticket| {
        nearbyticket.iter().all(|&v| {
            rules
                .iter()
                .any(|&(_, r1s, r1e, r2s, r2e)| (r1s <= v && v <= r1e) || (r2s <= v && v <= r2e))
        })
    });

    let mut candidates = vec![true; rules.len() * rules.len()];
    let mut r_lens = vec![rules.len(); rules.len()];

    for nearbyticket in nearbytickets.iter() {
        for (vpos, &v) in nearbyticket.iter().enumerate() {
            for (rpos, &(_, r1s, r1e, r2s, r2e)) in rules.iter().enumerate() {
                if !((r1s <= v && v <= r1e) || (r2s <= v && v <= r2e)) {
                    if mem::replace(&mut candidates[vpos + rules.len() * rpos], false) {
                        r_lens[rpos] -= 1;
                    }
                }
            }
        }
    }

    let mut associations = vec![usize::MAX; rules.len()];
    let mut associations_len = 0;
    while associations_len < rules.len() {
        for r in 0..rules.len() {
            if r_lens[r] == 1 {
                let v = candidates[r * rules.len()..(r + 1) * rules.len()]
                    .iter()
                    .position(|&b| b)
                    .unwrap();
                let prev = mem::replace(&mut associations[r], v);
                assert_eq!(prev, usize::MAX, "Can't solve");
                associations_len += 1;
                for r in 0..rules.len() {
                    if mem::replace(&mut candidates[v + r * rules.len()], false) {
                        r_lens[r] -= 1;
                    }
                }
            }
        }
    }

    rules
        .iter()
        .enumerate()
        .map(|(pos, (name, _, _, _, _))| (pos, name))
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(pos, _)| myticket[associations[pos]])
        .product()
}
