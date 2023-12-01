fn part_1(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += line.chars().find_map(|c| c.to_digit(10)).unwrap() * 10;
        sum += line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    }
    sum
}

fn part_2(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let patterns = [
            ("1", 1), ("one", 1),
            ("2", 2), ("two", 2),
            ("3", 3), ("three", 3),
            ("4", 4), ("four", 4),
            ("5", 5), ("five", 5),
            ("6", 6), ("six", 6),
            ("7", 7), ("seven", 7),
            ("8", 8), ("eight", 8),
            ("9", 9), ("nine", 9),
        ];
        let (_, a) = patterns.iter()
            .filter_map(|(p, n)| Some((line.find(p)?, n)))
            .min_by_key(|&(i, _)| i)
            .unwrap();
        let (_, b) = patterns.iter()
            .filter_map(|(p, n)| Some((line.rfind(p)?, n)))
            .max_by_key(|&(i, _)| i)
            .unwrap();
        sum += a * 10 + b;
    }
    sum
}

aoc::main!();
