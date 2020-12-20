#[allow(unused_imports)]
use super::prelude::*;
use itertools::{iproduct, izip};
type Input = HashMap<u64, Grid<bool>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|tile| {
            let (id, grid) = tile.splitn(2, "\n").collect_tuple().expect("Invalid input");
            let id = id[5..9].parse().expect("Invalid input");
            let grid = Grid::from_input_chars(grid, |c, _, _| c == '#');
            (id, grid)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut adj = HashMap::new();
    let mut edges = HashMap::new();

    for (&id, grid) in input {
        ArrayVec::from([
            iproduct!(0..10usize, 0..1),
            iproduct!(0..10usize, 9..10),
            iproduct!(0..1usize, 0..10),
            iproduct!(9..10usize, 0..10),
        ])
        .into_iter()
        .map(|i| i.map(|p| grid[p]).collect::<ArrayVec<[bool; 10]>>())
        .for_each(|edge| {
            edges.insert(edge.iter().copied().rev().collect(), id);
            if let Some(old) = edges.insert(edge, id) {
                *adj.entry(id).or_insert(0) += 1;
                *adj.entry(old).or_insert(0) += 1;
            }
        });
    }

    adj.into_iter()
        .filter(|&(_, v)| v == 2)
        .map(|(k, _)| k)
        .product()
}

fn rotate_grid(grid: &mut Grid<bool>) {
    let size = grid.width;
    for (x, y) in iproduct!(0..size / 2, 0..size / 2) {
        let base = x + y * size;
        grid.vec.swap(base, size - 1 - y + x * size);
        grid.vec.swap(base, size - 1 - x + (size - 1 - y) * size);
        grid.vec.swap(base, y + (size - 1 - x) * size);
    }
}

fn flip_grid(grid: &mut Grid<bool>) {
    let size = grid.width;
    for (x, y) in iproduct!(0..size / 2, 0..size) {
        grid.vec.swap(size - 1 - x + y * size, x + y * size);
    }
}

fn adj_vert(grid1: &Grid<bool>, grid2: &Grid<bool>) -> bool {
    izip!(iproduct!(0..10usize, 9..10), iproduct!(0..10usize, 0..1))
        .all(|(p1, p2)| grid1[p1] == grid2[p2])
}

fn adj_horiz(grid1: &Grid<bool>, grid2: &Grid<bool>) -> bool {
    izip!(iproduct!(9..10usize, 0..10), iproduct!(0..1usize, 0..10))
        .all(|(p1, p2)| grid1[p1] == grid2[p2])
}

pub fn part2(input: &Input) -> usize {
    let mut adj: HashMap<u64, ArrayVec<[u64; 4]>> = HashMap::new();
    let mut edges = HashMap::new();

    for (&id, grid) in input {
        ArrayVec::from([
            iproduct!(0..10usize, 0..1),
            iproduct!(0..10usize, 9..10),
            iproduct!(0..1usize, 0..10),
            iproduct!(9..10usize, 0..10),
        ])
        .into_iter()
        .map(|i| i.map(|p| grid[p]).collect::<ArrayVec<[bool; 10]>>())
        .for_each(|edge1| {
            let mut edge2 = edge1.clone();
            edge2.reverse();
            edges.insert(edge2, id);
            if let Some(old) = edges.insert(edge1, id) {
                adj.entry(id).or_default().push(old);
                adj.entry(old).or_default().push(id);
            }
        });
    }

    let mut id_pos_map = HashMap::new();
    let mut pos_grid_map = HashMap::new();

    let first_id = adj.keys().copied().next().unwrap();
    id_pos_map.insert(first_id, (0, 0));
    pos_grid_map.insert((0, 0), input[&first_id].clone());

    let mut queue = adj[&first_id]
        .iter()
        .map(|&next_id| (first_id, next_id))
        .collect::<Vec<_>>();

    while let Some((prev, next)) = queue.pop() {
        if !id_pos_map.contains_key(&next) {
            let prev_pos = id_pos_map[&prev];
            let prev_grid = &pos_grid_map[&prev_pos];
            let mut grid = input[&next].clone();
            for i in 0..8 {
                let dpos = match () {
                    _ if adj_vert(&prev_grid, &grid) => (0, 1),
                    _ if adj_vert(&grid, &prev_grid) => (0, -1),
                    _ if adj_horiz(&prev_grid, &grid) => (1, 0),
                    _ if adj_horiz(&grid, &prev_grid) => (-1, 0),
                    _ => {
                        rotate_grid(&mut grid);
                        if i == 3 {
                            flip_grid(&mut grid);
                        }
                        continue;
                    }
                };

                let new_pos = (prev_pos.0 + dpos.0, prev_pos.1 + dpos.1);
                pos_grid_map.insert(new_pos, grid);
                id_pos_map.insert(next, new_pos);

                queue.extend(adj[&next].iter().copied().map(|nextnext| (next, nextnext)));

                break;
            }
        }
    }

    let mut minx = isize::MAX;
    let mut maxx = isize::MIN;
    let mut miny = isize::MAX;
    let mut maxy = isize::MIN;
    for (x, y) in pos_grid_map.keys().copied() {
        minx = min(minx, x);
        maxx = max(maxx, x);
        miny = min(miny, y);
        maxy = max(maxy, y);
    }

    let dim_x = 8 * (maxx + 1 - minx) as usize;
    let dim_y = 8 * (maxy + 1 - miny) as usize;

    let mut final_grid = Grid {
        vec: vec![false; dim_x * dim_y],
        width: dim_x,
    };

    for (sx, sy) in iproduct!(minx..maxx + 1, miny..maxy + 1) {
        let square_grid = &pos_grid_map[&(sx, sy)];
        let sx = (sx - minx) as usize;
        let sy = (sy - miny) as usize;
        for (dx, dy) in iproduct!(0..8, 0..8) {
            final_grid[(8 * sx + dx, 8 * sy + dy)] = square_grid[(dx + 1, dy + 1)];
        }
    }

    let monster = b"                  # #    ##    ##    ### #  #  #  #  #  #   ";
    let monster_relpos = iproduct!(0..20, 0..3)
        .filter(|&(x, y)| monster[x + y * 20] == b'#')
        .collect::<Vec<_>>();

    let mut sea_monsters_points = HashSet::new();

    for i in 0..8 {
        let mut found = false;
        for (x, y) in iproduct!(0..final_grid.width - 19, 0..final_grid.height() - 2) {
            if monster_relpos
                .iter()
                .copied()
                .all(|(dx, dy)| final_grid[(x + dx, y + dy)])
            {
                found = true;
                sea_monsters_points.extend(monster_relpos.iter().map(|&(dx, dy)| (x + dx, y + dy)));
            }
        }
        if found {
            break;
        }
        rotate_grid(&mut final_grid);
        if i == 3 {
            flip_grid(&mut final_grid);
        }
    }

    final_grid.vec.iter().filter(|&&b| b).count() - sea_monsters_points.len()
}
