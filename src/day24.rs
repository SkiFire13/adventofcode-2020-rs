#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<Dir>>;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    E,
    W,
    NW,
    NE,
    SW,
    SE
}

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|line| {
        let mut path = Vec::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                'e' => path.push(Dir::E),
                'w' => path.push(Dir::W),
                's' => path.push(match chars.next() {
                    Some('e') => Dir::SE,
                    Some('w') => Dir::SW,
                    _ => panic!(),
                }),
                'n' => path.push(match chars.next() {
                    Some('e') => Dir::NE,
                    Some('w') => Dir::NW,
                    _ => panic!(),
                }),
                _ => panic!(),
            }
        }
        path
    }).collect()
}

pub fn part1(input: &Input) -> usize {
    let mut tiles = FxHashMap::default();
    for path in input {
        let mut pos = (0, 0);
        for &dir in path {
            match dir {
                Dir::E => pos.0 += 1,
                Dir::W => pos.0 -= 1,
                Dir::SE => pos.1 -= 1,
                Dir::SW => {
                    pos.1 -= 1;
                    pos.0 -= 1;
                }
                Dir::NW => pos.1 += 1,
                Dir::NE => {
                    pos.0 += 1;
                    pos.1 += 1;
                }
            }
        }

        let entry = tiles.entry(pos).or_insert(false);
        *entry = !*entry; 
    }

    tiles.iter().filter(|&(_, &v)| v).count()
}

pub fn part2(input: &Input) -> usize {
    let mut tiles = FxHashMap::default();
    for path in input {
        let mut pos = (0, 0);
        for &dir in path {
            match dir {
                Dir::E => pos.0 += 1,
                Dir::W => pos.0 -= 1,
                Dir::SE => pos.1 -= 1,
                Dir::SW => {
                    pos.1 -= 1;
                    pos.0 -= 1;
                }
                Dir::NW => pos.1 += 1,
                Dir::NE => {
                    pos.0 += 1;
                    pos.1 += 1;
                }
            }
        }

        let entry = tiles.entry(pos).or_insert(false);
        *entry = !*entry; 
    }

    let mut adj = FxHashMap::default();

    for _ in 0..100 {
        adj.clear();
        for (&pos, &black) in tiles.iter() {
            if black {
                adj.entry(pos).or_insert(0);
                *adj.entry((pos.0 + 1, pos.1)).or_insert(0) += 1;
                *adj.entry((pos.0 - 1, pos.1)).or_insert(0) += 1;
                *adj.entry((pos.0, pos.1 + 1)).or_insert(0) += 1;
                *adj.entry((pos.0, pos.1 - 1)).or_insert(0) += 1;
                *adj.entry((pos.0 + 1, pos.1 + 1)).or_insert(0) += 1;
                *adj.entry((pos.0 - 1, pos.1 - 1)).or_insert(0) += 1;
            }
        }

        for (&pos, &n_black) in adj.iter() {
            let was_black = tiles.get(&pos).copied().unwrap_or(false);
            if was_black && (n_black == 0 || n_black > 2) {
                tiles.entry(pos).and_modify(|is_black| *is_black = false);
            } else if !was_black && n_black == 2 {
                tiles.insert(pos, true);
            }
        }
    }

    tiles.iter().filter(|&(_, &v)| v).count()
}
