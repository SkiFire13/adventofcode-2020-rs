#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<char>;

pub fn input_generator(input: &str) -> Input {
    Grid {
        vec: input.lines()
            .flat_map(|line| line.chars())
            .collect(),
        width: input.lines().next().unwrap().len()
    }
}

fn encounters(grid: &Grid<char>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < grid.height() {
        if grid[(x % grid.width, y)] == '#' {
            count += 1;
        }
        x += dx;
        y += dy;
    }
    count
}

pub fn part1(input: &Input) -> usize {
    let grid = input;
    encounters(grid, 3, 1)
}

pub fn part2(input: &Input) -> usize {
    let grid = input;
    encounters(grid, 1, 1) *
    encounters(grid, 3, 1) *
    encounters(grid, 5, 1) *
    encounters(grid, 7, 1) *
    encounters(grid, 1, 2)
}
