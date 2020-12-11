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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Bounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_diag13: isize,
    max_diag13: isize,
    min_diag24: isize,
    max_diag24: isize,
}

impl Bounds {
    const UNBOUNDED: Bounds = Self {
        min_x: isize::MAX,
        max_x: isize::MIN,
        min_y: isize::MAX,
        max_y: isize::MIN,
        min_diag13: isize::MAX,
        max_diag13: isize::MIN,
        min_diag24: isize::MAX,
        max_diag24: isize::MIN,
    };

    fn initial_for_grid(grid: &Grid<CellPos>) -> Self {
        Self {
            min_x: 1,
            max_x: (grid.width - 1) as isize,
            min_y: 1,
            max_y: (grid.height() - 1) as isize ,
            min_diag13: 0,
            max_diag13: (grid.width + grid.height() - 2) as isize,
            min_diag24: (0 - grid.height() + 1) as isize,
            max_diag24: (grid.width - 1 - 0) as isize, 
        }
    }

    fn add_point(&mut self, (x, y): (usize, usize)) {
        let (x, y) = (x as isize, y as isize);
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
        self.min_diag13 = min(self.min_diag13, x + y);
        self.max_diag13 = max(self.max_diag13, x + y);
        self.min_diag24 = min(self.min_diag24, x - y);
        self.max_diag24 = max(self.max_diag24, x - y);
    }

    fn are_unbounded(&self) -> bool {
        self.min_x == isize::MAX
    }

    fn for_each(&self, mut f: impl FnMut(usize, usize)) {
        let mut y = self.min_y;
        while y <= self.max_y {
            let mut x = max(self.min_x, max(self.min_diag13 - y, self.min_diag24 + y));
            let end_x = min(self.max_x, min(self.max_diag13 - y, self.max_diag24 + y));
            while x <= end_x {
                f(x as usize, y as usize);
                x += 1;
            }
            y += 1;
        }
    }
}

fn evolve(
    input: &Grid<CellPos>,
    max_occupied: usize,
    sees_occupied: impl Fn(&Grid<CellPos>, usize, usize) -> usize,
    update_bounds: impl Fn(usize, usize, &mut Bounds)
) -> usize {
    let mut grid = input.clone();
    let mut new_grid = grid.clone();

    let mut bounds = Bounds::initial_for_grid(&grid);

    while !bounds.are_unbounded() {
        let mut new_bounds = Bounds::UNBOUNDED;

        bounds.for_each(|x, y| {
            if grid[(x, y)] != CellPos::Floor {
                let n_occupied = sees_occupied(&grid, x, y);

                new_grid[(x, y)] = match grid[(x, y)] {
                    CellPos::EmptySeat if n_occupied == 0 => {
                        update_bounds(x, y, &mut new_bounds);
                        CellPos::OccupiedSeat
                    }
                    CellPos::OccupiedSeat if n_occupied >= max_occupied => {
                        update_bounds(x, y, &mut new_bounds);
                        CellPos::EmptySeat
                    }
                    cellpos => cellpos,
                }
            }
        });

        bounds = new_bounds;
        mem::swap(&mut grid, &mut new_grid);
    }

    grid.vec
        .iter()
        .filter(|&&cellpos| cellpos == CellPos::OccupiedSeat)
        .count()
}

pub fn part1(input: &Input) -> usize {
    evolve(
        input, 
        4, 
        |grid, x, y| {
            let x = x as isize;
            let y = y as isize;
            [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)].iter()
                .filter(|&(dx, dy)| grid.get(((x + dx) as usize, (y + dy) as usize)) == Some(&CellPos::OccupiedSeat))
                .count()
        }, 
        |x, y, bounds| {
            let x = x as isize;
            let y = y as isize;
            for &(dx, dy) in &[(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                bounds.add_point(((x + dx) as usize, (y + dy) as usize))
            }
        }
    )
}

pub fn part2(input: &Input) -> usize {
    let grid = input;
    let width = grid.width;

    let neighbours = grid.vec
        .iter()
        .enumerate()
        .map(|(idx, &cell)| {
            let x = (idx % width) as isize;
            let y = (idx / width) as isize;
            match cell {
                CellPos::Floor => Default::default(),
                _ => [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
                    .iter()
                    .filter_map(|&(dx, dy)| {
                        let mut nx = x + dx;
                        let mut ny = y + dy;
                        while let Some(CellPos::Floor) = grid.iget((nx, ny)) {
                            nx += dx;
                            ny += dy;
                        }
                        grid.iget((nx, ny)).map(|_| (nx as usize, ny as usize))
                    })
                    .collect::<ArrayVec<[_; 8]>>()
            }
        })
        .collect::<Vec<_>>();

    evolve(
        grid, 
        5, 
        |grid, x, y| {
            neighbours[x + y * grid.width].iter()
                .filter(|&&pos| grid[pos] == CellPos::OccupiedSeat)
                .count()
        }, 
        |x, y, bounds| {
            for &(nx, ny) in &neighbours[x + y * grid.width] {
                bounds.add_point((nx, ny));
            }
        }
    )
}
