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

    let mut min_x = 1;
    let mut max_x = grid.width;
    let mut min_y = 1;
    let mut max_y = grid.height();

    let mut changed = true;

    while changed {
        changed = false;
        let mut new_min_x = usize::MAX;
        let mut new_max_x = 0;
        let mut new_min_y = usize::MAX;
        let mut new_max_y = 0;

        for y in min_y..max_y - 1 {
            for x in min_x..max_x - 1 {
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
                            new_min_x = min(new_min_x, x - 1);
                            new_max_x = max(new_max_x, x + 2);
                            new_min_y = min(new_min_y, y - 1);
                            new_max_y = max(new_max_y, y + 2);
                            CellPos::OccupiedSeat
                        }
                        CellPos::OccupiedSeat if n_occupied >= max_occupied => {
                            changed = true;
                            new_min_x = min(new_min_x, x - 1);
                            new_max_x = max(new_max_x, x + 2);
                            new_min_y = min(new_min_y, y - 1);
                            new_max_y = max(new_max_y, y + 2);
                            CellPos::EmptySeat
                        }
                        cellpos => cellpos,
                    }
                }
            }
        }

        min_x = new_min_x;
        max_x = new_max_x;
        min_y = new_min_y;
        max_y = new_max_y;
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
