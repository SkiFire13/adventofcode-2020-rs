#[allow(unused_imports)]
use super::prelude::*;
use itertools::{iproduct, izip};
type Input = HashMap<u64, Grid<bool>>;

pub fn input_generator(input: &str) -> Input {
    input
        .split("\n\n")
        .map(|tile| {
            let (id, grid) = tile.splitn(2, "\n").collect_tuple().unwrap();
            let id = id.strip_prefix("Tile ").unwrap();
            let id = id.strip_suffix(":").unwrap();
            let id = id.parse().unwrap();
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
        .for_each(|edge1| {
            let mut edge2 = edge1.clone();
            edge2.reverse();
            edges.insert(edge2, id);
            if let Some(old) = edges.insert(edge1, id) {
                assert!(old != id);
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
    assert!(grid.width == grid.height());
    for (x, y) in iproduct!(0..grid.width / 2, 0..grid.height() / 2) {
        grid.vec
            .swap(x + y * grid.width, grid.width - 1 - y + x * grid.width);
        grid.vec.swap(
            x + y * grid.width,
            grid.width - 1 - x + (grid.width - 1 - y) * grid.width,
        );
        grid.vec
            .swap(x + y * grid.width, y + (grid.width - 1 - x) * grid.width);
    }
}

fn flip_grid(grid: &mut Grid<bool>) {
    for (x, y) in iproduct!(0..grid.width / 2, 0..grid.height()) {
        grid.vec
            .swap(grid.width - 1 - x + y * grid.width, x + y * grid.width);
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
                let dpos = if adj_vert(&prev_grid, &grid) {
                    (0, 1)
                } else if adj_vert(&grid, &prev_grid) {
                    (0, -1)
                } else if adj_horiz(&prev_grid, &grid) {
                    (1, 0)
                } else if adj_horiz(&grid, &prev_grid) {
                    (-1, 0)
                } else {
                    if i == 8 {
                        panic!()
                    }
                    rotate_grid(&mut grid);
                    if i == 3 {
                        flip_grid(&mut grid);
                    }
                    continue;
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

    'outer: for i in 0..8 {
        for (x, y) in iproduct!(0..final_grid.width - 19, 0..final_grid.height() - 2) {
            let contains_monster = monster_relpos
                .iter()
                .all(|&(dx, dy)| final_grid[(x + dx, y + dy)]);
            if contains_monster {
                break 'outer;
            }
        }
        if i == 8 {
            panic!()
        }
        rotate_grid(&mut final_grid);
        if i == 3 {
            flip_grid(&mut final_grid);
        }
    }

    let mut sea_monsters_points = HashSet::new();
    for (x, y) in iproduct!(0..final_grid.width - 19, 0..final_grid.height() - 2) {
        let contains_monster = monster_relpos
            .iter()
            .copied()
            .all(|(dx, dy)| final_grid[(x + dx, y + dy)]);
        if contains_monster {
            sea_monsters_points.extend(monster_relpos.iter().map(|&(dx, dy)| (x + dx, y + dy)));
        }
    }

    final_grid.vec.iter().filter(|&&b| b).count() - sea_monsters_points.len()
}
