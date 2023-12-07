fn hand_rank(hand: &str, enable_jokers: bool) -> (u32, [usize; 5]) {
    let card_order = match enable_jokers {
        false => "AKQJT987865432",
        true => "AKQT987865432J",
    };
    let card_rank = |card| card_order.find(card).unwrap();

    let mut card_counts = [0; 14];
    let mut card_ranks = [0; 5];
    for (index, card) in hand.chars().enumerate() {
        let rank = card_rank(card);
        card_counts[rank] += 1;
        card_ranks[index] = rank;
    }

    if enable_jokers {
        let joker_index = card_rank('J');
        let jokers = card_counts[joker_index];
        card_counts[joker_index] = 0;
        card_counts.sort_unstable();
        *card_counts.last_mut().unwrap() += jokers;
    } else {
        card_counts.sort_unstable();
    }
    let rank = match card_counts {
        [.., 5] => 0,
        [.., 1, 4] => 1,
        [.., 2, 3] => 2,
        [.., 1, 1, 3] => 3,
        [.., 1, 2, 2] => 4,
        [.., 1, 1, 1, 2] => 5,
        [.., 1, 1, 1, 1, 1] => 6,
        _ => unreachable!(),
    };

    (rank, card_ranks)
}

fn total_winnings(plays: &str, enable_jokers: bool) -> u32 {
    let mut plays = plays.lines()
        .map(|play| {
            let (hand, bid) = play.split_once(' ').unwrap();
            (hand_rank(hand, enable_jokers), bid.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    plays.sort_unstable();
    plays.iter().rev().zip(1..).map(|((_, bid), rank)| bid * rank).sum()
}

fn part_1(input: String) -> u32 {
    total_winnings(&input, false)
}

fn part_2(input: String) -> u32 {
    total_winnings(&input, true)
}

aoc::main!();
