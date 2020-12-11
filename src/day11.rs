#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<CellPos>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CellPos {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

pub fn input_generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len() + 2;
    let mut vec = Vec::with_capacity(input.len() + 3 * width);

    vec.extend(iter::repeat(CellPos::Floor).take(width));
    for line in input.lines() {
        vec.push(CellPos::Floor);
        vec.extend(line.chars().map(|c| match c {
            '.' => CellPos::Floor,
            'L' => CellPos::EmptySeat,
            '#' => CellPos::OccupiedSeat,
            _ => panic!(),
        }));
        vec.push(CellPos::Floor);
    }
    vec.extend(iter::repeat(CellPos::Floor).take(width));
    
    Grid { vec, width }
}

fn evolve(
    input: &Grid<CellPos>,
    max_occupied: usize,
    mut sees_occupied: impl FnMut(&Grid<CellPos>, isize, isize, isize, isize) -> bool,
) -> usize {
    let mut grid = input.clone();
    let mut new_grid = grid.clone();

    let mut changed = true;
    while changed {
        changed = false;

        for y in 1..grid.height() - 1 {
            for x in 1..grid.width - 1 {
                if grid[(x, y)] != CellPos::Floor {
                    let n_occupied = [
                        (-1, -1),
                        (0, -1),
                        (1, -1),
                        (-1, 0),
                        (1, 0),
                        (-1, 1),
                        (0, 1),
                        (1, 1),
                    ]
                    .iter()
                    .copied()
                    .filter(|&(dx, dy)| sees_occupied(&grid, x as isize, y as isize, dx, dy))
                    .count();

                    new_grid[(x, y)] = match grid[(x, y)] {
                        CellPos::EmptySeat if n_occupied == 0 => {
                            changed = true;
                            CellPos::OccupiedSeat
                        }
                        CellPos::OccupiedSeat if n_occupied >= max_occupied => {
                            changed = true;
                            CellPos::EmptySeat
                        }
                        cellpos => cellpos,
                    }
                }
            }
        }

        mem::swap(&mut grid, &mut new_grid);
    }

    grid.vec
        .iter()
        .filter(|&&cellpos| cellpos == CellPos::OccupiedSeat)
        .count()
}

pub fn part1(input: &Input) -> usize {
    evolve(input, 4, |grid, x, y, dx, dy| {
        grid.get(((x + dx) as usize, (y + dy) as usize)) == Some(&CellPos::OccupiedSeat)
    })
}

pub fn part2(input: &Input) -> usize {
    evolve(input, 5, |grid, x, y, dx, dy| {
        let mut x = x + dx;
        let mut y = y + dy;
        while let Some(CellPos::Floor) = grid.iget((x, y)) {
            x += dx;
            y += dy;
        }
        grid.iget((x, y)) == Some(&CellPos::OccupiedSeat)
    })
}
