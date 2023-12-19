use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X, M, A, S
}

type Workflow<'s> = (Vec<(Category, Ordering, u32, &'s str)>, &'s str);
type Part = [u32; 4];

fn parse_category(category: &str) -> Category {
    match category {
        "x" => Category::X,
        "m" => Category::M,
        "a" => Category::A,
        "s" => Category::S,
        _ => panic!(),
    }
}

fn parse_ordering(ordering: &str) -> Ordering {
    match ordering {
        "<" => Ordering::Less,
        ">" => Ordering::Greater,
        _ => panic!(),
    }
}

fn parse_workflow(workflow: &str) -> (&str, Workflow) {
    let (name, rules) = workflow.split_once('{').unwrap();
    let rules = rules.strip_suffix('}').unwrap();
    let (rules, default) = rules.rsplit_once(',').unwrap();
    let rules = rules.split(',');
    let rules = rules
        .map(|rule| {
            let (predicate, target) = rule.split_once(':').unwrap();
            let category = parse_category(&predicate[0..1]);
            let ordering = parse_ordering(&predicate[1..2]);
            let value = predicate[2..].parse().unwrap();
            (category, ordering, value, target)
        })
        .collect();
    (name, (rules, default))
}

fn parse_part(part: &str) -> Part {
    part
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .fold(Part::default(), |mut part, field| {
            let (category, value) = field.split_once('=').unwrap();
            let category = parse_category(category);
            let value = value.parse().unwrap();
            part[category as usize] = value;
            part
        })
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows.lines().map(parse_workflow).collect();
    let parts = parts.lines().map(parse_part).collect();
    (workflows, parts)
}

fn part_1(input: String) -> u32 {
    let (workflows, parts) = parse_input(&input);

    let mut sum = 0;
    for part in parts {
        let mut workflow_name = "in";
        while let Some((rules, default)) = workflows.get(workflow_name) {
            workflow_name = rules.iter()
                .find_map(|&(category, ordering, value, target)| {
                    if part[category as usize].cmp(&value) == ordering {
                        Some(target)
                    } else {
                        None
                    }
                })
                .unwrap_or(default);
        }
        if workflow_name == "A" {
            sum += part.iter().sum::<u32>();
        }
    }
    sum
}

fn part_2(input: String) -> u64 {
    let (workflows, _) = parse_input(&input);
    let mut parts = vec![([1..4001, 1..4001, 1..4001, 1..4001], "in")];

    let mut sum = 0;
    while let Some((mut part, workflow_name)) = parts.pop() {
        if let Some((rules, default)) = workflows.get(workflow_name) {
            for &(category, ordering, value, target) in rules {
                let range = &part[category as usize];
                let (failed, passed) = match ordering {
                    Ordering::Less => (value..range.end, range.start..value),
                    Ordering::Greater => (range.start..value + 1, value + 1..range.end),
                    _ => panic!(),
                };
                
                let mut matched = part.clone();
                part[category as usize] = failed;
                matched[category as usize] = passed;
                if matched.iter().all(|r| !r.is_empty()) {
                    parts.push((matched, target));
                }
            }
            if part.iter().all(|r| !r.is_empty()) {
                parts.push((part, default));
            }
        } else if workflow_name == "A" {
            sum += part.iter().map(|r| r.len() as u64).product::<u64>();
        }
    }
    sum
}

aoc::main!();
