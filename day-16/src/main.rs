use std::collections::{HashMap, HashSet};

fn parse_grid(grid: &str) -> HashMap<(i32, i32), char> {
    grid.lines()
        .enumerate()
        .flat_map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Beam {
    fn fw(self) -> Self {
        let Self { x, y, dx, dy } = self;
        Self { x: x + dx, y: y + dy, dx, dy }
    }

    fn cw(self) -> Self {
        let Self { x, y, dx, dy } = self;
        Self { x, y, dx: -dy, dy: dx }
    }

    fn ccw(self) -> Self {
        let Self { x, y, dx, dy } = self;
        Self { x, y, dx: dy, dy: -dx }
    }
}

fn energized_tiles(grid: &HashMap<(i32, i32), char>, start: Beam) -> usize {
    let mut visited = HashSet::new();
    let mut energized = HashSet::new();
    let mut beams = vec![start];
    while let Some(beam) = beams.pop() {
        if !visited.insert(beam) {
            continue;
        }
        match grid.get(&(beam.x, beam.y)) {
            Some('.') => beams.push(beam.fw()),
            Some('/') => beams.push(if beam.dx != 0 { beam.ccw() } else { beam.cw() }.fw()),
            Some('\\') => beams.push(if beam.dy != 0 { beam.ccw() } else { beam.cw() }.fw()),
            Some('-') if beam.dx != 0 => beams.push(beam.fw()),
            Some('|') if beam.dy != 0 => beams.push(beam.fw()),
            Some('-') => beams.extend([beam.cw().fw(), beam.ccw().fw()]),
            Some('|') => beams.extend([beam.cw().fw(), beam.ccw().fw()]),
            None => continue,
            _ => panic!()
        }
        energized.insert((beam.x, beam.y));
    }
    energized.len()
}

fn part_1(input: String) -> usize {
    let grid = parse_grid(&input);
    energized_tiles(&grid, Beam { x: 0, y: 0, dx: 1, dy: 0 })
}

fn part_2(input: String) -> usize {
    let grid = parse_grid(&input);
    let min_x = grid.keys().map(|&(x, _)| x).min().unwrap();
    let max_x = grid.keys().map(|&(x, _)| x).max().unwrap();
    let min_y = grid.keys().map(|&(_, y)| y).min().unwrap();
    let max_y = grid.keys().map(|&(_, y)| y).max().unwrap();
    
    let mut max = 0;
    for x in min_x..max_x {
        max = max.max(energized_tiles(&grid, Beam { x, y: 0, dx: 0, dy: 1 }));
        max = max.max(energized_tiles(&grid, Beam { x, y: max_y, dx: 0, dy: -1 }));
    }
    for y in min_y..max_y {
        max = max.max(energized_tiles(&grid, Beam { x: 0, y, dx: 1, dy: 0 }));
        max = max.max(energized_tiles(&grid, Beam { x: max_x, y, dx: -1, dy: 0 }));
    }
    max
}

aoc::main!();
