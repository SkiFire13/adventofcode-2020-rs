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
            let (name, rest) = line.split(": ").collect_tuple().expect("Invalid input");
            let (r1, r2) = rest.split(" or ").collect_tuple().expect("Invalid input");
            let (r1s, r1e) = r1.split('-').collect_tuple().expect("Invalid input");
            let (r2s, r2e) = r2.split('-').collect_tuple().expect("Invalid input");
            (
                name,
                r1s.parse().expect("Invalid input"),
                r1e.parse().expect("Invalid input"),
                r2s.parse().expect("Invalid input"),
                r2e.parse().expect("Invalid input"),
            )
        })
        .collect();
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

    let mut association_candidates = vec![(0..rules.len()).collect_vec(); rules.len()];

    for nearbyticket in nearbytickets.iter() {
        for (vpos, &v) in nearbyticket.iter().enumerate() {
            let mut rpos = 0;
            let pos_candidates = &mut association_candidates[vpos];
            while rpos < pos_candidates.len() {
                let (_, r1s, r1e, r2s, r2e) = rules[pos_candidates[rpos]];
                if (r1s <= v && v <= r1e) || (r2s <= v && v <= r2e) {
                    rpos += 1;
                } else {
                    pos_candidates.swap_remove(rpos);
                }
            }
        }
    }

    let mut associations = HashMap::with_capacity(rules.len());
    while associations.len() < rules.len() {
        for (pos, candidates) in association_candidates.iter().enumerate() {
            if candidates.len() == 1 {
                associations.insert(candidates[0], pos);
            }
        }
        for candidates in association_candidates.iter_mut() {
            candidates.retain(|candidate| !associations.contains_key(candidate));
        }
    }

    rules
        .iter()
        .enumerate()
        .map(|(pos, (name, _, _, _, _))| (pos, name))
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(pos, _)| myticket[associations[&pos]])
        .product()
}
