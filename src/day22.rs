#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<u32>, Vec<u32>);

pub fn input_generator(input: &str) -> Input {
    let (first, second) = input.splitn(2, "\n\n").collect_tuple().expect("Invalid input");
    let (_, first) = first.splitn(2, "\n").collect_tuple().expect("Invalid input");
    let (_, second) = second.splitn(2, "\n").collect_tuple().expect("Invalid input");
    let first = first.lines().map(|line| line.parse().expect("Invalid input")).collect();
    let second = second.lines().map(|line| line.parse().expect("Invalid input")).collect();
    (first, second)
}

pub fn part1(input: &Input) -> u32 {
    let (first, second) = input;
    let mut first = first.iter().copied().collect::<VecDeque<_>>();
    let mut second = second.iter().copied().collect::<VecDeque<_>>();

    while let (Some(&f), Some(&s)) = (first.front(), second.front()) {
        first.pop_front();
        second.pop_front();
        if f > s {
            first.push_back(f);
            first.push_back(s);
        } else {
            second.push_back(s);
            second.push_back(f);
        }
    }

    first
        .into_iter()
        .chain(second)
        .rev()
        .enumerate()
        .map(|(pos, n)| (pos + 1) as u32 * n)
        .sum()
}

fn recursive_game(first: &mut VecDeque<u32>, second: &mut VecDeque<u32>) -> bool {
    let mut seen = HashSet::new();

    while let (Some(&f), Some(&s)) = (first.front(), second.front()) {
        if seen.insert((first.clone(), second.clone())) {
            first.pop_front();
            second.pop_front();

            let f_win = if first.len() >= f as usize && second.len() >= s as usize {
                recursive_game(
                    &mut first.iter().copied().take(f as usize).collect(),
                    &mut second.iter().copied().take(s as usize).collect(),
                )
            } else {
                f > s
            };

            if f_win {
                first.push_back(f);
                first.push_back(s);
            } else {
                second.push_back(s);
                second.push_back(f);
            }
        } else {
            return true;
        }
    }

    !first.is_empty()
}

pub fn part2(input: &Input) -> u32 {
    let (first, second) = input;
    let mut first = first.iter().copied().collect::<VecDeque<_>>();
    let mut second = second.iter().copied().collect::<VecDeque<_>>();

    let winner = if recursive_game(&mut first, &mut second) {
        first
    } else {
        second
    };

    winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(pos, n)| (pos + 1) as u32 * n)
        .sum()
}
