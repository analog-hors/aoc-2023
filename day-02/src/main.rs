fn part_1(input: String) -> u32 {
    let mut sum = 0;
    'iter: for line in input.lines() {
        let (game_str, subsets) = line.split_once(": ").unwrap();
        let game_id = game_str.strip_prefix("Game ").unwrap();
        let game_id = game_id.parse::<u32>().unwrap();

        for subset in subsets.split("; ") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for cubes in subset.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => red += count,
                    "green" => green += count,
                    "blue" => blue += count,
                    _ => panic!()
                }
            }
            if red > 12 || green > 13 || blue > 14 {
                continue 'iter;
            }
        }
        sum += game_id;
    }
    sum
}

fn part_2(input: String) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (_, subsets) = line.split_once(": ").unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in subsets.split("; ") {
            for cubes in subset.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => red = red.max(count),
                    "green" => green = green.max(count),
                    "blue" => blue = blue.max(count),
                    _ => panic!()
                }
            }
        }
        sum += red * green * blue;
    }
    sum
}

aoc::main!();
