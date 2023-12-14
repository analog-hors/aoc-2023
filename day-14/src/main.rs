use std::collections::HashMap;

type Cell = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Round,
    Square,
}

struct Grid<T> {
    grid: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> Grid<T> {
    fn get(&self, x: i32, y: i32) -> &T {
        assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        &self.grid[(y * self.width + x) as usize]
    }

    fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        &mut self.grid[(y * self.width + x) as usize]
    }
}

fn parse_input(input: &str) -> Grid<Tile> {
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        for tile in line.chars() {
            grid.push(match tile {
                '.' => Tile::Empty,
                'O' => Tile::Round,
                '#' => Tile::Square,
                _ => panic!(),
            });
        }
        width = line.len() as i32;
        height += 1;
    }
    Grid { grid, width, height }
}

fn part_1(input: String) -> i32 {
    let platform = parse_input(&input);
    let mut sum = 0;
    for x in 0..platform.width {
        let mut top = platform.height;
        for y in 0..platform.height {
            match platform.get(x, y) {
                Tile::Empty => {}
                Tile::Round => {
                    sum += top;
                    top -= 1;
                }
                Tile::Square => {
                    top = platform.height - 1 - y;
                }
            }
        }
    }
    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, Default)]
struct EndpointMap {
    n: i32,
    e: i32,
    s: i32,
    w: i32,
}

fn make_endpoint_map(grid: &Grid<Tile>) -> Grid<EndpointMap> {
    let mut map = Grid {
        grid: vec![EndpointMap::default(); (grid.width * grid.height) as usize],
        width: grid.width,
        height: grid.height,
    };

    for y in 0..grid.height {
        let mut endpoint = -1;
        for x in 0..grid.width {
            map.get_mut(x, y).w = endpoint;
            if *grid.get(x, y) == Tile::Square {
                endpoint = x;
            }
        }

        let mut endpoint = grid.width;
        for x in (0..grid.width).rev() {
            map.get_mut(x, y).e = endpoint;
            if *grid.get(x, y) == Tile::Square {
                endpoint = x;
            }
        }
    }


    for x in 0..grid.width {
        let mut endpoint = -1;
        for y in 0..grid.height {
            map.get_mut(x, y).n = endpoint;
            if *grid.get(x, y) == Tile::Square {
                endpoint = y;
            }
        }

        let mut endpoint = grid.height;
        for y in (0..grid.height).rev() {
            map.get_mut(x, y).s = endpoint;
            if *grid.get(x, y) == Tile::Square {
                endpoint = y;
            }
        }
    }

    map
}

fn get_round_rocks(grid: &Grid<Tile>) -> Vec<Cell> {
    (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| (x, y)))
        .filter(|&(x, y)| *grid.get(x, y) == Tile::Round)
        .collect()
}

fn map_rocks(map: &Grid<EndpointMap>, dir: Direction, rocks: &mut Vec<Cell>) {
    let mut endpoints = HashMap::new();
    for (x, y) in rocks.drain(..) {
        let map = map.get(x, y);
        let endpoint = match dir {
            Direction::N => (x, map.n),
            Direction::E => (map.e, y),
            Direction::S => (x, map.s),
            Direction::W => (map.w, y),
        };
        *endpoints.entry(endpoint).or_insert(0) += 1;
    }
    for ((mut x, mut y), c) in endpoints {
        for _ in 0..c {
            match dir {
                Direction::N => y += 1,
                Direction::E => x -= 1,
                Direction::S => y -= 1,
                Direction::W => x += 1,
            }
            rocks.push((x, y));
        }
    }
}

fn apply_cycles(map: &Grid<EndpointMap>, cycles: usize, rocks: &mut Vec<Cell>) {
    let mut state_map = HashMap::new();
    let mut states = Vec::new();
    for i in 0..cycles {
        rocks.sort_unstable();
        if let Some(cycle_start) = state_map.insert(rocks.clone(), i) {
            let cycle_size = i - cycle_start;
            let cycle_index = (cycles - cycle_start) % cycle_size;
            *rocks = states.swap_remove(cycle_start + cycle_index);
            break;
        }
        states.push(rocks.clone());

        for dir in [Direction::N, Direction::W, Direction::S, Direction::E] {
            map_rocks(&map, dir, rocks);
        }
    }
}

fn part_2(input: String) -> i32 {
    let platform = parse_input(&input);
    let map = make_endpoint_map(&platform);
    let mut rocks = get_round_rocks(&platform);
    apply_cycles(&map, 1_000_000_000, &mut rocks);
    rocks.iter().map(|(_, y)| platform.height - y).sum()
}

aoc::main!();
