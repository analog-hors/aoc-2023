use std::ops::Range;

fn parse_seed_nums(input: &str) -> impl Iterator<Item=u64> + '_ {
    input.lines()
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
}

fn parse_map_groups(input: &str) -> Vec<Vec<(u64, u64, u64)>> {
    input.split("\n\n")
        .skip(1)
        .map(|maps| {
            maps.lines()
                .skip(1)
                .map(|map| {
                    let mut nums = map.split_ascii_whitespace()
                        .map(|n| n.parse().unwrap());
                    let dest = nums.next().unwrap();
                    let src = nums.next().unwrap();
                    let len = nums.next().unwrap();
                    (dest, src, len)
                })
                .collect()
        })
        .collect()
}

fn part_1(input: String) -> u64 {
    let mut nums = parse_seed_nums(&input).collect::<Vec<_>>();
    let map_groups = parse_map_groups(&input);
    for map_group in &map_groups {
        for n in &mut nums {
            for &(dest, src, len) in map_group {
                if (src..src + len).contains(n) {
                    *n = *n - src + dest;
                    break;
                }
            }
        }
    }
    nums.into_iter().min().unwrap()
}

fn remap_range(map_group: &[(u64, u64, u64)], range: Range<u64>, remapped: &mut Vec<Range<u64>>) {
    let mut unmapped = vec![range];
    for &(dest, src, len) in map_group {
        let mut next = Vec::new();
        for range in unmapped {
            let overlap_start = range.start.max(src);
            let overlap_end = range.end.min(src + len);
            if overlap_start < overlap_end {
                let remapped_start = overlap_start - src + dest;
                let remapped_end = overlap_end - src + dest;
                remapped.push(remapped_start..remapped_end);
                if range.start < overlap_start {
                    next.push(range.start..overlap_start);
                }
                if overlap_end < range.end {
                    next.push(overlap_end..range.end);
                }
            } else {
                next.push(range);
            }
        }
        unmapped = next;
    }
    remapped.append(&mut unmapped);
}

fn part_2(input: String) -> u64 {
    let mut ranges = Vec::new();
    let mut seeds = parse_seed_nums(&input);
    while let (Some(start), Some(len)) = (seeds.next(), seeds.next()) {
        ranges.push(start..start + len);
    }

    let map_groups = parse_map_groups(&input);
    for map_group in &map_groups {
        let mut remapped = Vec::new();
        for range in ranges {
            remap_range(map_group, range, &mut remapped);
        }
        ranges = remapped;
    }
    ranges.iter().map(|r| r.start).min().unwrap()
}

aoc::main!();
