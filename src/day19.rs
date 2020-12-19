#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (HashMap<u32, Rule<'a>>, Vec<&'a str>);

#[derive(Clone, Debug)]
pub enum Rule<'a> {
    Literal(&'a str),
    Or(ArrayVec<[u32; 3]>, ArrayVec<[u32; 3]>),
}

pub fn input_generator(input: &str) -> Input {
    let (rules, tests) = input.split("\n\n").collect_tuple().expect("Invalid input");
    let rules = rules
        .lines()
        .map(|line| {
            let (n, rule) = line.split(": ").collect_tuple().expect("Invalid input");
            let n = n.parse().expect("Invalid input");
            let rule = if rule.starts_with('"') {
                Rule::Literal(
                    rule.strip_prefix('"')
                        .unwrap()
                        .strip_suffix('"')
                        .expect("Invalid input"),
                )
            } else {
                let (rs1, rs2) = rule.split(" | ").collect_tuple().unwrap_or((rule, ""));
                Rule::Or(
                    rs1.split_ascii_whitespace()
                        .map(|r| r.parse().expect("Invalid input"))
                        .collect(),
                    rs2.split_ascii_whitespace()
                        .map(|r| r.parse().expect("Invalid input"))
                        .collect(),
                )
            };
            (n, rule)
        })
        .collect();
    let tests = tests.lines().collect();
    (rules, tests)
}

fn match_rule<'i>(s: &'i str, rule: &Rule, rules: &HashMap<u32, Rule>) -> bool {
    let mut queue = Vec::new();
    queue.push((s, vec![rule]));

    while let Some((s, mut rules_stack)) = queue.pop() {
        if let Some(rule) = rules_stack.pop() {
            match rule {
                Rule::Literal(lit) => {
                    if let Some(s) = s.strip_prefix(*lit) {
                        queue.push((s, rules_stack));
                    }
                }
                Rule::Or(rs1, rs2) => {
                    if !rs2.is_empty() {
                        let mut rules_stack2 = rules_stack.clone();
                        rules_stack2.extend(rs2.iter().map(|r| &rules[r]).rev());
                        queue.push((s, rules_stack2));
                    }
                    let mut rules_stack1 = rules_stack;
                    rules_stack1.extend(rs1.iter().map(|r| &rules[r]).rev());
                    queue.push((s, rules_stack1));
                }
            }
        } else if s == "" {
            return true;
        }
    }

    false
}

pub fn part1(input: &Input) -> usize {
    let (rules, tests) = input;

    tests
        .iter()
        .filter(|s| match_rule(s, &rules[&0], &rules))
        .count()
}

pub fn part2(input: &Input) -> usize {
    let (rules, tests) = input;

    let mut rules = rules.clone();
    rules.insert(
        8,
        Rule::Or(
            [42].iter().copied().collect(),
            [42, 8].iter().copied().collect(),
        ),
    );
    rules.insert(
        11,
        Rule::Or(
            [42, 31].iter().copied().collect(),
            [42, 11, 31].iter().copied().collect(),
        ),
    );

    tests
        .iter()
        .filter(|s| match_rule(s, &rules[&0], &rules))
        .count()
}
