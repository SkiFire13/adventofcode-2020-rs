#[allow(unused_imports)]
use super::prelude::*;
type Input = [u8; 9];

pub fn input_generator(input: &str) -> Input {
    input
        .chars()
        .map(|c| c as u8 - b'0')
        .collect::<ArrayVec<[u8; 9]>>()
        .into_inner()
        .expect("Invalid input")
}

pub fn part1(input: &Input) -> u32 {
    let mut cups = *input;
    for _ in 0..100 {
        let current_cup = cups[0];
        let next_cups = [
            cups[1],
            cups[2],
            cups[3],
        ];
        let mut dest_cup = current_cup - 1;
        if dest_cup == 0 { dest_cup = cups.iter().copied().max().unwrap(); }
        while next_cups.contains(&dest_cup) {
            dest_cup -= 1;
            if dest_cup == 0 { dest_cup = cups.iter().copied().max().unwrap(); }
        }
        let dest_cup_pos = cups.iter().position(|&cup| cup == dest_cup).unwrap();
        cups[1..=dest_cup_pos].rotate_left(3);
        cups.rotate_left(1);
    }
    let pos1 = cups.iter().position(|&cup| cup == 1).unwrap();
    cups.rotate_left(pos1);
    cups[1..].iter().fold(0, |acc, &d| 10 * acc + (d as u32))
}

struct IndexLinkedList {
    nodes: Vec<u32>,
    front: usize,
    back: usize,
}

impl IndexLinkedList {
    fn pop_front(&mut self) -> u32 {
        let val = self.front as u32;
        self.front = mem::take(&mut self.nodes[self.front]) as usize;
        val
    }
    fn push_back(&mut self, val: u32) {
        self.nodes[self.back] = val;
        self.back = val as usize;
    }
    fn pop_after(&mut self, val: u32) -> u32 {
        let next = mem::take(&mut self.nodes[val as usize]);
        self.nodes[val as usize] = mem::take(&mut self.nodes[next as usize]);
        next
    }
    fn push_after(&mut self, val: u32, after: u32) {
        self.nodes[val as usize] = mem::replace(&mut self.nodes[after as usize], val);
    }
}

pub fn part2(input: &Input) -> u64 {
    let mut nodes = vec![0; 1_000_000 + 1];

    let mut prev = 0;
    for &d in input {
        nodes[prev] = d as u32;
        prev = d as usize;
    }
    for i in 10..1_000_000 + 1 {
        nodes[prev] = i;
        prev = i as usize;
    }

    let mut list = IndexLinkedList {
        nodes,
        front: input[0] as usize,
        back: 1_000_000,
    };

    for _ in 0..10_000_000 {
        let current_cup = list.pop_front();
        let next_cups = [
            list.pop_front(),
            list.pop_front(),
            list.pop_front(),
        ];

        let mut dest_cup = current_cup - 1;
        if dest_cup == 0 { dest_cup = 1_000_000; }
        while next_cups.contains(&dest_cup) {
            dest_cup -= 1;
            if dest_cup == 0 { dest_cup = 1_000_000; }
        }

        list.push_back(current_cup);
        list.push_after(next_cups[2], dest_cup);
        list.push_after(next_cups[1], dest_cup);
        list.push_after(next_cups[0], dest_cup);
    }

    list.pop_after(1) as u64 * list.pop_after(1) as u64
}
