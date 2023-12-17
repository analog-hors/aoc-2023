use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

type Cell = (i32, i32);
type Node = (i32, i32, bool);
type Grid = HashMap<Cell, u32>;

fn parse_grid(grid: &str) -> Grid {
    grid.lines()
        .enumerate()
        .flat_map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect()
}

fn neighbours(grid: &Grid, min_shift: i32, max_shift: i32, (x, y, y_axis): Node, mut visit: impl FnMut(Node, u32)) {
    for direction in [-1, 1] {
        let mut total_weight = 0;
        for shift in 1..=max_shift {
            let (nx, ny) = match y_axis {
                false => (x + shift * direction, y),
                true => (x, y + shift * direction),
            };
            let Some(weight) = grid.get(&(nx, ny)) else {
                break;
            };
            total_weight += weight;
            if shift >= min_shift {
                visit((nx, ny, !y_axis), total_weight);
            }
        }
    }
}

fn min_cost(grid: &Grid, min_shift: i32, max_shift: i32) -> u32 {
    let mut costs = HashMap::<Node, u32>::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), (0, 0, false)));
    to_visit.push((Reverse(0), (0, 0, true)));
    while let Some((Reverse(cost), node)) = to_visit.pop() {
        neighbours(grid, min_shift, max_shift, node, |neighbour, weight|  {
            let n_cost = cost + weight;
            let old_cost = costs.entry(neighbour).or_insert(u32::MAX);
            if n_cost < *old_cost {
                *old_cost = n_cost;
                to_visit.push((Reverse(n_cost), neighbour));
            }
        });
    }

    let (tx, ty) = *grid.keys().max().unwrap();
    let x_cost = *costs.get(&(tx, ty, false)).unwrap();
    let y_cost = *costs.get(&(tx, ty, true)).unwrap();
    x_cost.min(y_cost)
}

fn part_1(input: String) -> u32 {
    min_cost(&parse_grid(&input), 1, 3)
}

fn part_2(input: String) -> u32 {
    min_cost(&parse_grid(&input), 4, 10)
}

aoc::main!();
