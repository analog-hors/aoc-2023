use std::ops::Range;
use std::collections::HashSet;

fn parse_galaxies(input: &str) -> HashSet<(i32, i32)> {
    let mut galaxies = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            if cell == '#' {
                galaxies.insert((x as i32, y as i32));
            }
        }
    }
    galaxies
}

fn expansion_ranges(galaxies: &HashSet<(i32, i32)>, axis: impl Fn(&(i32, i32)) -> i32) -> Vec<Range<i32>> {
    let mut points = galaxies.iter().map(axis).collect::<Vec<_>>();
    points.sort_unstable();
    points.windows(2).map(|p| p[0] + 1..p[1]).filter(|r| !r.is_empty()).collect()
}

fn expansion_size_between(expansions: &[Range<i32>], s: i32, e: i32) -> i32 {
    // this could be binary search but this is more than fast enough
    expansions.iter().filter(|r| s < r.start && r.end <= e).map(|r| r.len() as i32).sum()
}

fn axis_distance(expansions: &[Range<i32>], x1: i32, x2: i32, expansion_factor: i32) -> i32 {
    let expand = expansion_size_between(expansions, x1, x2);
    x2 - x1 + expand * (expansion_factor - 1)
}

fn galaxy_pair_distance_sum(input: &str, expansion_factor: i32) -> i64 {
    let galaxies = parse_galaxies(&input);
    let x_expansions = expansion_ranges(&galaxies, |&(x, _)| x);
    let y_expansions = expansion_ranges(&galaxies, |&(_, y)| y);
    let mut sum = 0;
    for &(ax, ay) in &galaxies {
        for &(bx, by) in &galaxies {
            if (ax, ay) < (bx, by) {
                let (x1, x2) = (ax.min(bx), ax.max(bx));
                let (y1, y2) = (ay.min(by), ay.max(by));
                sum += axis_distance(&x_expansions, x1, x2, expansion_factor) as i64;
                sum += axis_distance(&y_expansions, y1, y2, expansion_factor) as i64;
            }
        }
    }
    sum
}

fn part_1(input: String) -> i64 {
    galaxy_pair_distance_sum(&input, 2)
}

fn part_2(input: String) -> i64 {
    galaxy_pair_distance_sum(&input, 1_000_000)
}

aoc::main!();
