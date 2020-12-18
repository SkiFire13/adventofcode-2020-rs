#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Token>;

#[derive(Clone, Copy)]
pub enum Token {
    Num(u64),
    OpPlus,
    OpTimes,
    ParOpen,
    ParClose,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .flat_map(|line| {
            let mut chars = line.chars().peekable();
            iter::once(Token::OpPlus)
                .chain(iter::once(Token::ParOpen))
                .chain(iter::from_fn(move || {
                    while let Some(' ') = chars.peek() { chars.next(); }
                    Some(match chars.next()? {
                        '+' => Token::OpPlus,
                        '*' => Token::OpTimes,
                        '(' => Token::ParOpen,
                        ')' => Token::ParClose,
                        c @ '0'..='9' => Token::Num(
                            chars
                                .peeking_take_while(|&c| matches!(c, '0'..='9'))
                                .map(|c| c as u64 - '0' as u64)
                                .fold(c as u64 - '0' as u64, |acc, d| 10 * acc + d),
                        ),
                        _ => panic!("Invalid input"),
                    })
                }))
                .chain(iter::once(Token::ParClose))
        })
        .skip(1)
        .collect()
}

fn next_value<I: Iterator<Item = Token>>(tokens: &mut I, eval: fn(&mut I) -> u64) -> u64 {
    match tokens.next() {
        Some(Token::Num(n)) => n,
        Some(Token::ParOpen) => eval(tokens),
        _ => panic!("Invalid token"),
    }
}

pub fn part1(input: &Input) -> u64 {
    fn eval(tokens: &mut impl Iterator<Item = Token>) -> u64 {
        let mut acc = next_value(tokens, eval);

        loop {
            match tokens.next() {
                Some(Token::OpPlus) => acc += next_value(tokens, eval),
                Some(Token::OpTimes) => acc *= next_value(tokens, eval),
                Some(Token::ParClose) | None => return acc,
                _ => panic!("Invalid token"),
            }
        }
    }

    let tokens = input;
    eval(&mut tokens.iter().copied())
}

pub fn part2(input: &Input) -> u64 {
    fn eval(tokens: &mut impl Iterator<Item = Token>) -> u64 {
        let mut acc = 1;
        let mut curr = next_value(tokens, eval);

        loop {
            match tokens.next() {
                Some(Token::OpPlus) => curr += next_value(tokens, eval),
                Some(Token::OpTimes) => {
                    acc *= curr;
                    curr = next_value(tokens, eval);
                },
                Some(Token::ParClose) | None => return acc * curr,
                _ => panic!("Invalid token"),
            };
        }
    }

    let tokens = input;
    eval(&mut tokens.iter().copied())
}
