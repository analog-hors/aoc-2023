use std::collections::{HashMap, HashSet, VecDeque};

type Cell = (i32, i32);

#[derive(Debug, Clone)]
struct Grid<T> {
    grid: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> Grid<T> {
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.grid[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&mut self.grid[(y * self.width + x) as usize])
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> (Grid<char>, Cell) {
    let mut width = 0;
    let mut height = 0;
    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
                grid.push('.');
            } else {
                grid.push(c);
            }
        }
        width = line.len() as i32;
        height += 1;
    }
    (Grid { grid, width, height }, start)
}

fn part_1(input: String) -> usize {
    let (grid, start) = parse_input(&input);
    (0..64)
        .fold(HashSet::from([start]), |current, _| {
            current.into_iter()
                .flat_map(|(x, y)| [
                    (x + 1, y),
                    (x - 1, y),
                    (x, y + 1),
                    (x, y - 1),
                ])
                .filter(|&(x, y)| grid.get(x, y) == Some(&'.'))
                .collect()
        })
        .len()
}

fn distance_map(grid: &Grid<char>, start: Cell) -> [[Grid<Option<u32>>; 3]; 3] {
    let mut distances = [(); 3].map(|_| [(); 3].map(|_| {
        Grid {
            width: grid.width,
            height: grid.height,
            grid: vec![None; grid.grid.len()],
        }
    }));

    let mut to_visit = VecDeque::from([(start, 0)]);
    while let Some(((x, y), steps)) = to_visit.pop_front() {
        let (dx, gx) = (x.div_euclid(grid.width), x.rem_euclid(grid.width));
        let (dy, gy) = (y.div_euclid(grid.height), y.rem_euclid(grid.height));
        
        if dx < -1 || dx > 1 || dy < -1 || dy > 1 {
            continue;
        }

        if grid.get(gx, gy) != Some(&'.') {
            continue;
        }

        let slot = distances[(dy + 1) as usize][(dx + 1) as usize]
            .get_mut(gx, gy).unwrap();
        if slot.is_none() {
            *slot = Some(steps);
            to_visit.extend([
                ((x + 1, y), steps + 1),
                ((x - 1, y), steps + 1),
                ((x, y + 1), steps + 1),
                ((x, y - 1), steps + 1),
            ]);
        }
    }
    distances
}

const STEPS: u32 = 26501365;

fn part_2(input: String) -> u64 {
    let (grid, (sx, sy)) = parse_input(&input);
    let map = distance_map(&grid, (sx, sy));

    let mut cache = HashMap::new();
    let mut grid_valid_plots = |gx: i32, gy: i32| {
        let ox = gx.clamp(-1, 1);
        let oy = gy.clamp(-1, 1);
        let map = &map[(oy + 1) as usize][(ox + 1) as usize];
        let grid_distance = (gx - ox).abs() * grid.width + (gy - oy).abs() * grid.height;
        let parity = gx % 2 == gy % 2;

        *cache.entry((ox, oy, parity, grid_distance)).or_insert_with(|| {
            let mut valid = 0;
            for y in 0..grid.height {
                for x in 0..grid.width {
                    if let Some(distance) = *map.get(x, y).unwrap() {
                        let distance = distance + grid_distance as u32;
                        if distance <= STEPS && (STEPS - distance) % 2 == 0 {
                            valid += 1;
                        }
                    }
                }
            }
            valid
        })
    };

    let gsx = (sx - STEPS as i32) / grid.width - 1;
    let gex = (sx + STEPS as i32) / grid.width + 1;
    let gsy = (sy - STEPS as i32) / grid.height - 1;
    let gey = (sy + STEPS as i32) / grid.height + 1;

    let mut valid = 0;
    for x in gsx..gex {
        valid += grid_valid_plots(x, 0) as u64;
    }
    for y in gsy..gey {
        if y != 0 {
            valid += grid_valid_plots(0, y) as u64;
        }
    }

    let mut size = 0;
    let ne1 = grid_valid_plots(-1, -1);
    let ne2 = grid_valid_plots(-2, -1);
    let nw1 = grid_valid_plots(1, -1);
    let nw2 = grid_valid_plots(2, -1);
    let se1 = grid_valid_plots(-1, 1);
    let se2 = grid_valid_plots(-2, 1);
    let sw1 = grid_valid_plots(1, 1);
    let sw2 = grid_valid_plots(2, 1);
    for gy in gsy..0 {
        for x in -size..0 {
            let grid_valids = grid_valid_plots(x, gy);
            valid += grid_valids as u64;
            if grid_valids == ne1 {
                let remaining = -x - 1;
                valid += (remaining / 2 * ne1) as u64;
                valid += ((remaining / 2 + remaining % 2) * ne2) as u64;
                break;
            }
        }
        for x in (1..=size).rev() {
            let grid_valids = grid_valid_plots(x, gy);
            valid += grid_valids as u64;
            if grid_valids == nw1 {
                let remaining = x - 1;
                valid += (remaining / 2 * nw1) as u64;
                valid += ((remaining / 2 + remaining % 2) * nw2) as u64;
                break;
            }
        }
        size += 1;
    }
    for gy in 1..gey {
        size -= 1;
        for x in -size..0 {
            let grid_valids = grid_valid_plots(x, gy);
            valid += grid_valids as u64;
            if grid_valids == se1 {
                let remaining = -x - 1;
                valid += (remaining / 2 * se1) as u64;
                valid += ((remaining / 2 + remaining % 2) * se2) as u64;
                break;
            }
        }
        for x in (1..=size).rev() {
            let grid_valids = grid_valid_plots(x, gy);
            valid += grid_valids as u64;
            if grid_valids == sw1 {
                let remaining = x - 1;
                valid += (remaining / 2 * sw1) as u64;
                valid += ((remaining / 2 + remaining % 2) * sw2) as u64;
                break;
            }
        }
    }
    valid
}

aoc::main!();
