#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Vec<Rule<'a>>, Vec<&'a str>);

#[derive(Clone, Debug)]
pub enum Rule<'a> {
    Literal(&'a str),
    Seq(ArrayVec<[u8; 3]>),
    Or(ArrayVec<[u8; 3]>, ArrayVec<[u8; 3]>),
}

pub fn input_generator(input: &str) -> Input {
    fn parse_rule_seq(rule: &str) -> ArrayVec<[u8; 3]> {
        rule.split_ascii_whitespace()
            .map(|r| r.parse().expect("Invalid input"))
            .collect()
    }

    let (raw_rules, tests) = input.split("\n\n").collect_tuple().expect("Invalid input");
    let mut rules = vec![Rule::Literal(""); 256];
    for line in raw_rules.lines() {
        let (n, rule) = line.split(": ").collect_tuple().expect("Invalid input");
        let n = n.parse::<usize>().expect("Invalid input");
        let rule = if rule.starts_with('"') {
            Rule::Literal(
                rule.strip_prefix('"')
                    .unwrap()
                    .strip_suffix('"')
                    .expect("Invalid input"),
            )
        } else if let Some((rs1, rs2)) = rule.split(" | ").collect_tuple() {
            Rule::Or(parse_rule_seq(rs1), parse_rule_seq(rs2))
        } else {
            Rule::Seq(parse_rule_seq(rule))
        };
        rules[n] = rule;
    }

    let tests = tests.lines().collect();
    (rules, tests)
}

struct ChunkStack<'a, T> {
    chunk: &'a [T],
    next: Option<&'a ChunkStack<'a, T>>,
}

const INITIAL_STACK: ChunkStack<'static, u8> = ChunkStack {
    chunk: &[0],
    next: None,
};

impl<'a, T> ChunkStack<'a, T> {
    fn append(&'a self, chunk: &'a [T]) -> Self {
        let next = Some(self);
        Self { chunk, next }
    }
    fn pop(&mut self) -> Option<&'a T> {
        loop {
            if let Some((elem, rest)) = self.chunk.split_first() {
                self.chunk = rest;
                return Some(elem);
            } else {
                let next = self.next?;
                self.chunk = next.chunk;
                self.next = next.next;
            }
        }
    }
}

fn match_rule(s: &str, mut next: ChunkStack<u8>, rules: &[Rule]) -> bool {
    match next.pop().map(|&r| &rules[r as usize]) {
        Some(Rule::Literal(lit)) => s
            .strip_prefix(lit)
            .map_or(false, |s| match_rule(s, next, rules)),
        Some(Rule::Seq(seq)) => match_rule(s, next.append(seq), rules),
        Some(Rule::Or(seq1, seq2)) => {
            match_rule(s, next.append(seq1), rules) || match_rule(s, next.append(seq2), rules)
        }
        None => s == "",
    }
}

pub fn part1(input: &Input) -> usize {
    let (rules, tests) = input;

    tests
        .iter()
        .filter(|s| match_rule(s, INITIAL_STACK, &rules))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let (rules, tests) = input;

    let mut rules = rules.clone();
    rules[8] = Rule::Or(
        [42].iter().copied().collect(),
        [42, 8].iter().copied().collect(),
    );
    rules[11] = Rule::Or(
        [42, 31].iter().copied().collect(),
        [42, 11, 31].iter().copied().collect(),
    );

    tests
        .iter()
        .filter(|s| match_rule(s, INITIAL_STACK, &rules))
        .count()
}
