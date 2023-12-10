use std::collections::{HashMap, HashSet};

type Cell = (i32, i32);
type Tile = Option<[Cardinal; 2]>;
type Grid = HashMap<Cell, Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cardinal {
    N, E, S, W
}

impl Cardinal {
    fn flipped(&self) -> Self {
        match self {
            Self::N => Self::S,
            Self::E => Self::W,
            Self::S => Self::N,
            Self::W => Self::E,
        }
    }

    fn apply(&self, (x, y): Cell) -> Cell {
        match self {
            Self::N => (x, y - 1),
            Self::E => (x + 1, y),
            Self::S => (x, y + 1),
            Self::W => (x - 1, y),
        }
    }
}

fn parse_input(input: &str) -> (Grid, Cell) {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    for (y, row) in input.lines().enumerate() {
        for (x, tile_char) in row.chars().enumerate() {
            let cell = (x as i32, y as i32);
            let tile = match tile_char {
                '|' => Some([Cardinal::N, Cardinal::S]),
                '-' => Some([Cardinal::E, Cardinal::W]),
                'L' => Some([Cardinal::N, Cardinal::E]),
                'J' => Some([Cardinal::N, Cardinal::W]),
                '7' => Some([Cardinal::S, Cardinal::W]),
                'F' => Some([Cardinal::S, Cardinal::E]),
                '.' => None,
                'S' => None,
                _ => panic!(),
            };
            map.insert(cell, tile);
            if tile_char == 'S' {
                start = cell;
            }
        }
    }
    (map, start)
}

fn can_enter(grid: &Grid, cell: Cell, dir: Cardinal) -> bool {
    if let Some(Some([a, b])) = grid.get(&cell) {
        return a.flipped() == dir || b.flipped() == dir;
    }
    false
}

fn infer_start(grid: &mut Grid, start: Cell) {
    let mut tile = [Cardinal::N; 2];
    let mut index = 0;
    for dir in [Cardinal::N, Cardinal::E, Cardinal::S, Cardinal::W] {
        if can_enter(&grid, dir.apply(start), dir) {
            tile[index] = dir;
            index += 1;
        }
    }
    assert_eq!(index, 2);
    grid.insert(start, Some(tile));
}

fn loop_cells(grid: &Grid, start: Cell) -> Vec<Cell> {
    let mut cells = Vec::new();
    let mut current = start;
    while current != start || cells.is_empty() {
        let tile = grid.get(&current).unwrap().unwrap();
        for dir in tile {
            let next = dir.apply(current);
            if Some(&next) != cells.last() && can_enter(&grid, next, dir) {
                cells.push(current);
                current = next;
                break;
            }
        }
    }
    cells.push(current);
    cells
}

fn part_1(input: String) -> usize {
    let (mut grid, start) = parse_input(&input);
    infer_start(&mut grid, start);
    loop_cells(&grid, start).len() / 2
}

fn part_2(input: String) -> i32 {
    let (mut grid, start) = parse_input(&input);
    infer_start(&mut grid, start);
    let loop_cells = loop_cells(&grid, start);

    let min_x = loop_cells.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = loop_cells.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = loop_cells.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = loop_cells.iter().map(|&(_, y)| y).max().unwrap();

    let loop_cells = loop_cells.into_iter().collect::<HashSet<_>>();
    let mut enclosed = 0;
    for y in min_y..=max_y {
        let mut pipes = (min_x..=max_x)
            .filter(|&x| loop_cells.contains(&(x, y)))
            .map(|x| (x, grid.get(&(x, y)).unwrap().unwrap()));
        let segments = std::iter::from_fn(|| {
            let (x, pipe) = pipes.next()?;
            if !pipe.contains(&Cardinal::E) {
                return Some((x..=x, true));
            }

            let (end_x, end) = (&mut pipes)
                .find(|(_, p)| !p.contains(&Cardinal::E))
                .unwrap();
            let flips = end.contains(&Cardinal::N) != pipe.contains(&Cardinal::N);
            Some((x..=end_x, flips))
        });
        let mut inside = false;
        let mut prev = min_x..=min_x;
        for (segment, flips) in segments {
            if inside {
                enclosed += segment.start() - prev.end() - 1;
            }
            inside ^= flips;
            prev = segment;
        }
    }
    enclosed
}

aoc::main!();
