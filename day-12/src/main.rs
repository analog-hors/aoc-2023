use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Block {
    damaged: u128,
    len: u32,
}

fn parse_springs(springs: &str) -> Vec<Block> {
    springs.split('.')
        .filter(|s| !s.is_empty())
        .map(|b| Block {
            damaged: b.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(|(i, _)| 1 << i)
                .fold(0, |a, c| a | c),
            len: b.len() as u32,
        })
        .collect()
}

fn parse_runs(runs: &str) -> Vec<u32> {
    runs.split(',').map(|n| n.parse().unwrap()).collect()
}

fn enumerate_placements<'a>(
    block: &'a Block,
    runs: &'a [u32],
    placement: u128,
    cache: &mut HashMap<(u32, &'a [u32]), u64>,
    f: &mut impl FnMut(&'a [u32]) -> u64
) -> u64 {
    assert_eq!(placement & (1 << block.len) - 1, placement, "{}", block.len);

    let mut start = u128::BITS - placement.leading_zeros();
    if start != 0 {
        start += 1;
    }

    let filled = (1 << start) - 1;
    if filled & !placement & block.damaged != 0 {
        return 0;
    }

    if let Some(&n) = cache.get(&(start, runs)) {
        return n;
    }

    let mut sum = 0;
    if placement & block.damaged == block.damaged {
        sum += f(runs);
    }
    if let Some((run, runs)) = runs.split_first() {
        if block.len >= run - 1 {
            let run_bits = (1 << run) - 1;
            for shift in start..block.len - (run - 1) {
                let placement = placement | (run_bits << shift);
                sum += enumerate_placements(block, runs, placement, cache, f);
            }
        }
    }
    cache.insert((start, runs), sum);
    sum
}

fn possibilities<'a>(blocks: &'a [Block], runs: &'a [u32], cache: &mut HashMap<(&'a [Block], &'a [u32]), u64>) -> u64 {
    if let Some(&n) = cache.get(&(blocks, runs)) {
        return n;
    }
    let n = match blocks.split_first() {
        Some((block, blocks)) => enumerate_placements(
            block,
            runs,
            0,
            &mut HashMap::new(),
            &mut |runs| possibilities(blocks, runs, cache)
        ),
        None if runs.is_empty() => 1,
        None => 0,
    };
    cache.insert((blocks, runs), n);
    n
}

fn part_1(input: String) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let (springs, runs) = line.split_once(' ').unwrap();
        let blocks = parse_springs(springs);
        let runs = parse_runs(runs);
        sum += possibilities(&blocks, &runs, &mut HashMap::new());
    }
    sum
}

fn part_2(input: String) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let (springs, runs) = line.split_once(' ').unwrap();
        let blocks = parse_springs(&[springs; 5].join("?"));
        let runs = parse_runs(runs).repeat(5);
        sum += possibilities(&blocks, &runs, &mut HashMap::new());
    }
    sum
}

aoc::main!();
