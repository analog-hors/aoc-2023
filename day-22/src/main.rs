use std::ops::Range;

#[derive(Debug, Clone)]
struct Brick {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
}

fn parse_bricks(bricks: &str) -> Vec<Brick> {
    bricks.lines()
        .map(|brick| {
            let (start, end) = brick.split_once('~').unwrap();
            let start = start.split(',').map(|n| n.parse().unwrap());
            let end = end.split(',').map(|n| n.parse::<i32>().unwrap());
            let mut ranges = start.zip(end).map(|(s, e)| s..e + 1);
            Brick {
                x: ranges.next().unwrap(),
                y: ranges.next().unwrap(),
                z: ranges.next().unwrap(),
            }
        })
        .collect()
}

fn collides(a: &Brick, b: &Brick) -> bool {
    let x_overlap = !(b.x.start >= a.x.end || a.x.start >= b.x.end);
    let y_overlap = !(b.y.start >= a.y.end || a.y.start >= b.y.end);
    let z_overlap = !(b.z.start >= a.z.end || a.z.start >= b.z.end);
    x_overlap && y_overlap && z_overlap
}

fn apply_gravity(bricks: &mut [Brick]) -> u32 {
    let mut fallen_bricks = 0;
    bricks.sort_unstable_by_key(|b| b.z.start);
    for i in 0..bricks.len() {
        let (brick, below) = bricks[..i + 1].split_last_mut().unwrap();
        let init_z = brick.z.start;
        while brick.z.start > 0 && !below.iter().any(|b| collides(b, brick)) {
            brick.z.start -= 1;
            brick.z.end -= 1;
        }
        brick.z.start += 1;
        brick.z.end += 1;
        if brick.z.start != init_z {
            fallen_bricks += 1;
        }
    }
    fallen_bricks
}

fn part_1(input: String) -> usize {
    let mut bricks = parse_bricks(&input);
    apply_gravity(&mut bricks);
    (0..bricks.len())
        .filter(|&i| {
            let mut bricks = bricks.clone();
            bricks.swap_remove(i);
            apply_gravity(&mut bricks) == 0
        })
        .count()
}

fn part_2(input: String) -> u32 {
    let mut bricks = parse_bricks(&input);
    apply_gravity(&mut bricks);
    (0..bricks.len())
        .map(|i| {
            let mut bricks = bricks.clone();
            bricks.swap_remove(i);
            apply_gravity(&mut bricks)
        })
        .sum()
}

aoc::main!();
