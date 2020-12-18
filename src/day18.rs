#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<Token>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Num(i64),
    Op(Op),
    ParOpen,
    ParClose,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Plus,
    Times,
}

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let mut tokens = Vec::new();
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                tokens.push(match c {
                    '+' => Token::Op(Op::Plus),
                    '*' => Token::Op(Op::Times),
                    '(' => Token::ParOpen,
                    ')' => Token::ParClose,
                    ' ' => continue,
                    c @ '0'..='9' => {
                        let mut acc = (c as u8 - b'0') as i64;
                        while let Some(&c @ '0'..='9') = chars.peek() {
                            chars.next();
                            acc = 10 * acc + (c as u8 - b'0') as i64;
                        }
                        Token::Num(acc)
                    }
                    _ => panic!("Invalid input"),
                });
            }
            tokens
        })
        .collect()
}

pub fn part1(input: &Input) -> i64 {
    fn eval(expr: &mut impl Iterator<Item = Token>) -> i64 {
        let mut acc = match expr.next() {
            Some(Token::Num(n)) => n,
            Some(Token::ParOpen) => eval(expr),
            _ => panic!("Invalid token"),
        };

        loop {
            let op = match expr.next() {
                Some(Token::Op(Op::Plus)) => |a, b| a + b,
                Some(Token::Op(Op::Times)) => |a, b| a * b,
                Some(Token::ParClose) | None => return acc,
                _ => panic!("Invalid token"),
            };
            let n = match expr.next() {
                Some(Token::Num(n)) => n,
                Some(Token::ParOpen) => eval(expr),
                _ => panic!("Invalid token"),
            };
            acc = op(acc, n);
        }
    }

    let exprs = input;
    exprs
        .iter()
        .map(|expr| eval(&mut expr.iter().copied()))
        .sum()
}

pub fn part2(input: &Input) -> i64 {
    fn eval(expr: &mut impl Iterator<Item = Token>) -> i64 {
        let mut stack = vec![match expr.next() {
            Some(Token::Num(n)) => n,
            Some(Token::ParOpen) => eval(expr),
            _ => panic!(),
        }];

        loop {
            match expr.next() {
                Some(Token::Op(Op::Plus)) => {
                    let n = match expr.next() {
                        Some(Token::Num(n)) => n,
                        Some(Token::ParOpen) => eval(expr),
                        _ => panic!("Invalid token"),
                    };
                    *stack.last_mut().unwrap() += n;
                }
                Some(Token::Op(Op::Times)) => {
                    stack.push(match expr.next() {
                        Some(Token::Num(n)) => n,
                        Some(Token::ParOpen) => eval(expr),
                        _ => panic!("Invalid token"),
                    });
                }
                Some(Token::ParClose) | None => return stack.into_iter().product(),
                _ => panic!("Invalid token"),
            };
        }
    }

    let exprs = input;
    exprs
        .iter()
        .map(|expr| eval(&mut expr.iter().copied()))
        .sum()
}
