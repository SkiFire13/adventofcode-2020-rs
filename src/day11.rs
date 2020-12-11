#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<CellPos>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CellPos {
    EmptySeat,
    OccupiedSeat,
    Floor
}

pub fn input_generator(input: &str) -> Input {
    
    Grid {
        vec: input.lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '.' => CellPos::Floor,
                'L' => CellPos::EmptySeat,
                '#' => CellPos::OccupiedSeat,
                _ => panic!(),
            })
            .collect(),
        width: input.lines().next().unwrap().len()
    }
}

fn evolve(
    input: &Grid<CellPos>, 
    max_occupied_seats: usize,
    mut sees_occupied: impl FnMut(&Grid<CellPos>, isize, isize, isize, isize) -> bool,
) -> usize {
    let mut grid = input.clone();
    let mut new_grid = grid.clone();
    
    let mut changed = true;
    while changed {
        changed = false;

        for x in 0..grid.width {
            for y in 0..grid.height() {
                let nearly_occupied = {
                    let x = x as isize;
                    let y = y as isize;
                    (-1..=1)
                        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                        .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
                        .filter(|(dx, dy)| !(dx + x < 0 || dy + y < 0) )
                        .filter(|&(dx, dy)| sees_occupied(&grid, x, y, dx, dy))
                        .count()
                };

                match grid[(x, y)] {
                    CellPos::EmptySeat => {
                        new_grid[(x, y)] = CellPos::EmptySeat;
                        if nearly_occupied == 0 {
                            new_grid[(x, y)] = CellPos::OccupiedSeat;
                            changed = true;
                        }
                    }
                    CellPos::OccupiedSeat => {
                        new_grid[(x, y)] = CellPos::OccupiedSeat;
                        if nearly_occupied >= max_occupied_seats {
                            new_grid[(x, y)] = CellPos::EmptySeat;
                            changed = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        mem::swap(&mut grid, &mut new_grid);
    }

    grid.vec.iter().filter(|&&cellpos| cellpos == CellPos::OccupiedSeat).count()
}

pub fn part1(input: &Input) -> usize {
    evolve(input, 4, |grid, x, y, dx, dy| {
        grid.get(((x+dx)as usize, (y+dy) as usize)) == Some(&CellPos::OccupiedSeat)
    })
}

pub fn part2(input: &Input) -> usize {
    evolve(input, 5, |grid, x, y, dx, dy| {
        let mut x = x+dx;
        let mut y = y+dy;
        let mut found = false;
        while x >= 0 && y >= 0 && !found {
            match grid.get((x as usize, y as usize)) {
                Some(CellPos::OccupiedSeat) => found = true,
                Some(CellPos::EmptySeat) => break,
                None => break,
                _ => {
                    x += dx;
                    y += dy;
                }
            }
        }
        found
    })
}
