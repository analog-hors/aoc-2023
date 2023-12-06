fn ways_to_beat_record(time: u64, record: u64) -> usize {
    (1..time).filter(|held| held * (time - held) > record).count()
}

fn part_1(input: String) -> usize {
    let (times, records) = input.trim_end().split_once('\n').unwrap();
    let times = times.split_ascii_whitespace().skip(1).map(|n| n.parse().unwrap());
    let records = records.split_ascii_whitespace().skip(1).map(|n| n.parse().unwrap());
    times.zip(records).map(|(t, r)| ways_to_beat_record(t, r)).product()
}

fn part_2(input: String) -> usize {
    let (time, record) = input.trim_end().split_once('\n').unwrap();
    let time = time.strip_prefix("Time:").unwrap().replace(' ', "").parse().unwrap();
    let record = record.strip_prefix("Distance:").unwrap().replace(' ', "").parse().unwrap();
    ways_to_beat_record(time, record)
}

aoc::main!();
