fn card_matches(card_str: &str) -> usize {
    let (_, numbers) = card_str.split_once(": ").unwrap();
    let (winning_nums, our_nums) = numbers.split_once(" | ").unwrap();
    let winning_nums = winning_nums.split_ascii_whitespace().collect::<Vec<_>>();
    our_nums.split_ascii_whitespace().filter(|n| winning_nums.contains(n)).count()
}

fn part_1(input: String) -> u32 {
    input.lines().map(card_matches).filter(|&m| m > 0).map(|m| 1 << m - 1).sum()
}

fn part_2(input: String) -> u32 {
    let mut cards = input.lines()
        .map(|c| (1, card_matches(c)))
        .collect::<Vec<_>>();
    for i in 0..cards.len() {
        let (count, matches) = cards[i];
        for j in 0..matches {
            cards[i + 1 + j].0 += count;
        }
    }
    cards.iter().map(|(c, _)| c).sum()
}

aoc::main!();
