#[allow(unused_imports)]
use super::prelude::*;
type Input = (u64, u64);

pub fn input_generator(input: &str) -> Input {
    let (card_pub_key, door_pub_key) = input.lines().collect_tuple().expect("Invalid input");
    let card_pub_key = card_pub_key.parse().expect("Invalid input");
    let door_pub_key = door_pub_key.parse().expect("Invalid input");
    (card_pub_key, door_pub_key)
}

fn trasform(subj_num: u64, loop_size: u64) -> u64 {
    let mut acc = 1;
    let mut exp = loop_size;
    let mut base = subj_num;

    while exp != 0 {
        if exp % 2 == 1 {
            acc *= base;
            acc %= 20201227;
        }
        base *= base;
        base %= 20201227;
        exp /= 2;
    }
    
    acc
}

fn find_loop_size(target: u64) -> u64 {
    let mut acc = 1;

    for loop_size in 1.. {
        acc *= 7;
        acc %= 20201227;
        if acc == target {
            return loop_size;
        }
    }

    unreachable!()
}

pub fn part1(input: &Input) -> u64 {
    let &(card_pub_key, door_pub_key) = input;
    let card_loop_size = find_loop_size(card_pub_key);
    trasform(door_pub_key, card_loop_size)
}
