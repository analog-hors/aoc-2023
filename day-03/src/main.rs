use std::ops::Range;
use std::collections::HashMap;

fn all_nums(grid: &[&str]) -> Vec<(i32, Range<i32>, u32)> {
    let mut nums = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        let ends = row
            .match_indices(|c: char| !c.is_ascii_digit())
            .map(|(e, _)| e)
            .chain(std::iter::once(row.len()));

        let mut start = 0;
        for end in ends {
            if start < end {
                nums.push((
                    y as i32,
                    start as i32..end as i32,
                    row[start..end].parse().unwrap(),
                ));
            }
            start = end + 1;
        }
    }
    nums
}

fn neighbours(x: i32, y: i32) -> [(i32, i32); 8] {
    [
        (x + 1, y + 1),
        (x + 1, y    ),
        (x + 1, y - 1),
        (x    , y + 1),
        (x    , y - 1),
        (x - 1, y + 1),
        (x - 1, y    ),
        (x - 1, y - 1),
    ]
}

fn part_1(input: String) -> u32 {
    let grid = input.lines().collect::<Vec<_>>();
    let cell = |x: i32, y: i32| Some(
        *grid
            .get(usize::try_from(y).ok()?)?
            .as_bytes()
            .get(usize::try_from(x).ok()?)?
    );

    all_nums(&grid).iter()
        .filter(|(y, xs, _)| {
            xs.clone().any(|x| {
                neighbours(x, *y).iter()
                    .filter_map(|&(nx, ny)| cell(nx, ny))
                    .any(|c| !c.is_ascii_digit() && c != b'.')
            })
        })
        .map(|(_, _, n)| n)
        .sum()
}

fn part_2(input: String) -> u32 {
    let grid = input.lines().collect::<Vec<_>>();
    let cell = |x: i32, y: i32| Some(
        *grid
            .get(usize::try_from(y).ok()?)?
            .as_bytes()
            .get(usize::try_from(x).ok()?)?
    );
    
    let mut gears = HashMap::new();
    for (y, xs, n) in all_nums(&grid) {
        for (nx, ny) in xs.clone().flat_map(|x| neighbours(x, y)) {
            if cell(nx, ny) == Some(b'*') {
                let (c, r) = gears.entry((nx, ny)).or_insert((0, 1));
                *c += 1;
                *r *= n;
                break;
            }
        }
    }
    
    gears.values().filter(|&&(c, _)| c >= 2).map(|(_, n)| n).sum()
}

aoc::main!();
