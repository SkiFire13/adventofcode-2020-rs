#[allow(unused_imports)]
use super::prelude::*;
use itertools::iproduct;
type Input = Grid<bool>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| match c {
        '#' => true,
        '.' => false,
        _ => panic!("Invalid input"),
    })
}

fn solve<Point, Actives, Neighbours>(
    create_storage: impl Fn() -> Vec<bool>,
    index: impl Fn(Point) -> usize,
    bounds: impl Fn(i8) -> Actives,
    neighbours: fn(Point) -> Neighbours,
) -> usize
where
    Point: Copy + std::fmt::Debug,
    Actives: Iterator<Item = Point>,
    Neighbours: Iterator<Item = Point>,
{
    let mut space = create_storage();
    let mut adj = vec![0u8; space.len()];

    for i in 0..6 {
        adj.iter_mut().for_each(|s| *s = 0);
        for p in bounds(i + 1) {
            if space[index(p)] {
                for n in neighbours(p) {
                    adj[index(n)] += 1;
                }
            }
        }
        for p in bounds(i + 1) {
            let near_actives = adj[index(p)];
            space[index(p)] = near_actives == 3 || (space[index(p)] && near_actives == 2);
        }
    }

    space.iter().filter(|&&b| b).count()
}

pub fn part1(input: &Input) -> usize {
    let dims = [input.width, input.height(), 1];
    let dims_prod = |n| dims.iter().map(|&d| d + 2 * 7).take(n).product::<usize>();

    let index = |(x, y, z)| {
        (x + 7) as usize * dims_prod(0)
            + (y + 7) as usize * dims_prod(1)
            + (z + 7) as usize * dims_prod(2)
    };

    solve(
        || {
            let mut storage = vec![false; dims_prod(usize::MAX)];
            for (x, y) in iproduct!(0..input.width, 0..input.height()) {
                storage[index((x as i8, y as i8, 0))] = input[(x, y)];
            }
            storage
        },
        index,
        |r| {
            iproduct!(
                -r..r + dims[0] as i8,
                -r..r + dims[1] as i8,
                -r..r + dims[2] as i8
            )
        },
        |(x, y, z)| {
            iproduct!(-1..2, -1..2, -1..2)
                .filter(|&d| d != (0, 0, 0))
                .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
        },
    )
}

pub fn part2(input: &Input) -> usize {
    let dims = [input.width, input.height(), 1, 1];
    let dims_prod = |n| dims.iter().map(|&d| d + 2 * 7).take(n).product::<usize>();

    let index = |(x, y, z, w)| {
        (x + 7) as usize * dims_prod(0)
            + (y + 7) as usize * dims_prod(1)
            + (z + 7) as usize * dims_prod(2)
            + (w + 7) as usize * dims_prod(3)
    };

    solve(
        || {
            let mut storage = vec![false; dims_prod(usize::MAX)];
            for (x, y) in iproduct!(0..input.width, 0..input.height()) {
                storage[index((x as i8, y as i8, 0, 0))] = input[(x, y)];
            }
            storage
        },
        index,
        |r| {
            iproduct!(
                -r..r + dims[0] as i8,
                -r..r + dims[1] as i8,
                -r..r + dims[2] as i8,
                -r..r + dims[3] as i8
            )
        },
        |(x, y, z, w)| {
            iproduct!(-1..2, -1..2, -1..2, -1..2)
                .filter(|&d| d != (0, 0, 0, 0))
                .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
        },
    )
}
