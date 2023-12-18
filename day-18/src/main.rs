fn parse_waypoints(plan: &str, parse_line: impl Fn(&str) -> (char, i64)) -> Vec<(i64, i64)> {
    plan.lines()
        .map(parse_line)
        .scan((0, 0), |(x, y), (d, n)| {
            match d {
                'U' => *y += n,
                'D' => *y -= n,
                'L' => *x -= n,
                'R' => *x += n,
                _ => panic!(),
            }
            Some((*x, *y))
        })
        .collect()
}

fn trench_size(waypoints: &[(i64, i64)]) -> i64 {
    let get = |i| {
        let prev = if i > 0 { i - 1 } else { waypoints.len() - 1 };
        let next = if i < waypoints.len() - 1 { i + 1 } else { 0 };
        let (px, py) = waypoints[prev];
        let (nx, ny) = waypoints[next];
        let (x, y) = waypoints[i];
        let sx = if py > y || y > ny { x + 1 } else { x };
        let sy = if px < x || x < nx { y + 1 } else { y };
        (sx, sy)
    };

    let mut sum = 0;
    let (mut px, mut py) = get(waypoints.len() - 1);
    for i in 0..waypoints.len() {
        let (x, y) = get(i);
        sum += px * y;
        sum -= py * x;
        px = x;
        py = y;
    }
    sum.abs() / 2
}

fn part_1(input: String) -> i64 {
    let waypoints = parse_waypoints(&input, |line| {
        let mut parts = line.split_ascii_whitespace();
        let dir = parts.next().unwrap().chars().next().unwrap();
        let dist = parts.next().unwrap().parse().unwrap();
        (dir, dist)
    });
    trench_size(&waypoints)
}

fn part_2(input: String) -> i64 {
    let waypoints = parse_waypoints(&input, |line| {
        let (_, hex_part) = line.rsplit_once('#').unwrap();
        let dist = i64::from_str_radix(&hex_part[..5], 16).unwrap();
        let dir = match hex_part.chars().nth(5).unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => panic!(),
        };
        (dir, dist)
    });
    trench_size(&waypoints)
}

aoc::main!();
