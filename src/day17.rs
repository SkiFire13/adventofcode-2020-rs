#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i8, i8)>;

pub fn input_generator(input: &str) -> Input {
    input.lines().enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().filter_map(move |(x, c)| match c {
            '#' => Some((x as i8, y as i8)),
            '.' => None,
            _ => panic!(),
        }))
        .collect()
}

fn solve<Point: Hash + Eq + Copy + Send + Sync, Neighbours: Iterator<Item=Point>>(
    input: &[(i8, i8)],
    mapper: fn((i8, i8)) -> Point,
    neighbours: fn(Point) -> Neighbours
) -> usize {
    let mut actives = input.iter().copied().map(mapper).collect::<HashSet<_>>();
    let mut new_actives = HashSet::new();

    for _ in 0..6 {
        new_actives.clear();
        new_actives.par_extend(
            actives
                .par_iter()
                .flat_map_iter(|&p| neighbours(p).chain(iter::once(p)))
                .filter(|&p| {
                    let near_actives = neighbours(p)
                        .filter(|&p| actives.contains(&p))
                        .count();
                    let was_active = actives.contains(&p);
                    (was_active && (near_actives == 2 || near_actives == 3)) || (!was_active && near_actives == 3)
                })
        );
        mem::swap(&mut new_actives, &mut actives);
    }

    actives.len()
}

pub fn part1(input: &Input) -> usize {
    fn neighbours3d((x, y, z): (i8, i8, i8)) -> impl Iterator<Item = (i8, i8, i8)> {
        (-1..2)
            .flat_map(|dx| (-1..2).map(move |dy| (dx, dy)))
            .flat_map(|(dx, dy)| (-1..2).map(move |dz| (dx, dy, dz)))
            .filter(|&(dx, dy, dz)| !(dx == 0 && dy == 0 && dz == 0))
            .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
    }

    solve(
        input,
        |(x, y)| (x, y, 0),
        neighbours3d
    )
}

pub fn part2(input: &Input) -> usize {
    fn neighbours4d((x, y, z, w): (i8, i8, i8, i8)) -> impl Iterator<Item = (i8, i8, i8, i8)> {
        (-1..2)
            .flat_map(|dx| (-1..2).map(move |dy| (dx, dy)))
            .flat_map(|(dx, dy)| (-1..2).map(move |dz| (dx, dy, dz)))
            .flat_map(|(dx, dy, dz)| (-1..2).map(move |dw| (dx, dy, dz, dw)))
            .filter(|&(dx, dy, dz, dw)| !(dx == 0 && dy == 0 && dz == 0 && dw == 0))
            .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
    }

    solve(
        input,
        |(x, y)| (x, y, 0, 0),
        neighbours4d
    )
}
