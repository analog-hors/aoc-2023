fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |a, c| a.wrapping_add(c).wrapping_mul(17))
}

fn part_1(input: String) -> u32 {
    input.trim().split(',').map(|s| hash(s) as u32).sum()
}

fn part_2(input: String) -> u32 {
    const BOX: Vec<(String, u32)> = Vec::new();
    let mut boxes = [BOX; 256];
    for step in input.trim().split(',') {
        if let Some((label, focal_length)) = step.split_once('=') {
            let focal_length = focal_length.parse::<u32>().unwrap();
            let lenses = &mut boxes[hash(label) as usize];
            match lenses.iter_mut().find(|(l, _)| l == label) {
                Some((_, old)) => *old = focal_length,
                None => lenses.push((label.to_owned(), focal_length)),
            }
        } else if let Some(label) = step.strip_suffix('-') {
            boxes[hash(label) as usize].retain(|(l, _)| l != label);
        } else {
            panic!("invalid step: {}", step);
        }
    }

    let mut power = 0;
    for (b, lenses) in boxes.iter().enumerate() {
        for (i, (_, l)) in lenses.iter().enumerate() {
            power += (b as u32 + 1) * (i as u32 + 1) * l;
        }
    }
    power
}

aoc::main!();
