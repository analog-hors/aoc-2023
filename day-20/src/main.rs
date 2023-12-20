use std::collections::{HashMap, VecDeque};

enum Module<'s> {
    FlipFlop {
        state: bool,
        outputs: Vec<&'s str>,
    },
    Conjunction {
        state: HashMap<&'s str, bool>,
        outputs: Vec<&'s str>,
    },
}

fn parse_modules(modules_str: &str) -> (Vec<&str>, HashMap<&str, Module>) {
    let mut broadcaster = Vec::new();
    let mut modules = HashMap::new();
    for module in modules_str.lines() {
        let (name, outputs) = module.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").collect();
        if name == "broadcaster" {
            broadcaster = outputs;
            continue;
        }
        if let Some(name) = name.strip_prefix('%') {
            modules.insert(name, Module::FlipFlop { state: false, outputs });
            continue;
        }
        if let Some(name) = name.strip_prefix('&') {
            modules.insert(name, Module::Conjunction { state: HashMap::new(), outputs });
            continue;
        }
        panic!("invalid module '{}'", module);
    }

    let mut input_map = HashMap::new();
    for (&name, module) in modules.iter() {
        let (Module::FlipFlop { outputs, .. } | Module::Conjunction { outputs, .. }) = module;
        for &output in outputs {
            input_map.entry(output).or_insert_with(Vec::new).push(name);
        }
    }
    for (name, module) in modules.iter_mut() {
        if let (Module::Conjunction { state, .. }, Some(inputs)) = (module, input_map.get(name)) {
            *state = inputs.iter().map(|&o| (o, false)).collect();
        }
    }
    for module in &broadcaster {
        if let Some(Module::Conjunction { state, .. }) = modules.get_mut(module) {
            state.insert("broadcaster", false);
        }
    }

    (broadcaster, modules)
}

fn broadcast<'s>(
    broadcaster: &[&'s str],
    modules: &mut HashMap<&'s str, Module<'s>>,
    mut listener: impl FnMut(&'s str, &'s str, bool)
) {
    let mut pulses = broadcaster.iter()
        .map(|&t| ("broadcaster", t, false))
        .collect::<VecDeque<_>>();
    while let Some((source, target, high)) = pulses.pop_front() {
        listener(source, target, high);
        match modules.get_mut(target) {
            Some(Module::FlipFlop { state, outputs }) if !high => {
                *state ^= true;
                pulses.extend(outputs.iter().map(|&n| (target, n, *state)));
            }
            Some(Module::Conjunction { state, outputs }) => {
                *state.get_mut(source).unwrap() = high;
                let output = !state.values().all(|&p| p);
                pulses.extend(outputs.iter().map(|&n| (target, n, output)));
            }
            _ => {}
        }
    }
}

fn part_1(input: String) -> u32 {
    let (broadcaster, mut modules) = parse_modules(&input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;
    for _ in 0..1000 {
        low_pulses += 1;
        broadcast(&broadcaster, &mut modules, |_, _, high| {
            match high {
                false => low_pulses += 1,
                true => high_pulses += 1,
            }
        });
    }
    low_pulses * high_pulses
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a * b / gcd(a, b)
    }
}

fn part_2(input: String) -> u64 {
    let (broadcaster, mut modules) = parse_modules(&input);
    let (conjunction, total_counters) = modules.iter()
        .find_map(|(&name, module)| match module {
            Module::Conjunction { state, outputs }
                if outputs.contains(&"rx") => Some((name, state.len())),
            _ => None,
        })
        .unwrap();

    let mut counter_sizes = HashMap::new();
    for presses in 1.. {
        broadcast(&broadcaster, &mut modules, |source, target, high| {
            if target == conjunction && high {
                counter_sizes.entry(source).or_insert(presses);
            }
        });
        if counter_sizes.len() == total_counters {
            break;
        }
    }
    counter_sizes.into_values().reduce(lcm).unwrap()
}

aoc::main!();
