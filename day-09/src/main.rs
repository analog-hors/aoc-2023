fn extrapolated_sum(input: &str, backwards: bool) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut nums = line.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if backwards {
            nums.reverse();
        }
        while !nums.is_empty() {
            for i in 0..nums.len() - 1 {
                nums[i] = nums[i + 1] - nums[i];
            }
            sum += nums.pop().unwrap();
        }
    }
    sum
}

fn part_1(input: String) -> i32 {
    extrapolated_sum(&input, false)
}

fn part_2(input: String) -> i32 {
    extrapolated_sum(&input, true)
}

aoc::main!();
