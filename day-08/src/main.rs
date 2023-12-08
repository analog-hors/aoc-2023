use std::collections::HashMap;

fn parse_node(node: &str) -> (&str, (&str, &str)) {
    let (name, children) = node.split_once(" = ").unwrap();
    let (left, right) = children.split_once(", ").unwrap();
    let left = left.strip_prefix("(").unwrap();
    let right = right.strip_suffix(")").unwrap();
    (name, (left, right))
}

fn part_1(input: String) -> usize {
    let (path, graph) = input.split_once("\n\n").unwrap();
    let graph = graph.lines().map(parse_node).collect::<HashMap<_, _>>();

    let mut current = "AAA";
    for (steps, direction) in path.chars().cycle().enumerate() {
        if current == "ZZZ" {
            return steps;
        }
        let (left, right) = graph.get(current).unwrap();
        current = match direction {
            'L' => left,
            'R' => right,
            _ => panic!(),
        };
    }

    unreachable!()
}

fn visited<'g>(graph: &'g HashMap<&str, (&str, &str)>, path: &str, start: &'g str) -> (Vec<&'g str>, usize) {
    let mut current = start;
    let mut visited = Vec::new();
    let mut index = HashMap::new();
    loop {
        for (step, direction) in path.chars().enumerate() {
            if let Some(&cycle_start) = index.get(&(step, current)) {
                return (visited, cycle_start);
            }
            index.insert((step, current), visited.len());
            visited.push(current);
            let (left, right) = graph.get(current).unwrap();
            current = match direction {
                'L' => left,
                'R' => right,
                _ => panic!(),
            };
        }
    }
}

//TODO this is not strictly correct; it skips all nodes before the start of the cycle.
// Z-nodes tend to exist within the cycle with overwhelming probability, so this works out,
// but it's not technically correct.
struct ZNodeIter {
    indices: Vec<usize>,
    index: usize,
    cycle_size: usize,
}

impl ZNodeIter {
    pub fn new(visited: &[&str], cycle_start: usize) -> Self {
        let indices = visited.iter()
            .enumerate()
            .skip(cycle_start)
            .filter(|(_, n)| n.ends_with('Z'))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let cycle_size = visited.len() - cycle_start;
        Self { indices, cycle_size, index: 0 }
    }
}

impl Iterator for ZNodeIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index % self.indices.len();
        let cycles = self.index / self.indices.len();
        self.index += 1;
        Some(self.indices[index] + cycles * self.cycle_size)
    }
}

fn part_2(input: String) -> usize {
    let (path, graph) = input.split_once("\n\n").unwrap();
    let graph = graph.lines().map(parse_node).collect::<HashMap<_, _>>();

    let mut z_node_iters = graph.keys()
        .filter(|n| n.ends_with('A'))
        .map(|start| visited(&graph, path, start))
        .map(|(visited, cycle_start)| ZNodeIter::new(&visited, cycle_start).peekable())
        .collect::<Vec<_>>();

    loop {
        let mut all_equal = true;
        let target = z_node_iters.iter_mut()
            .map(|z| *z.peek().unwrap())
            .max()
            .unwrap();
        for z_iter in &mut z_node_iters {
            while *z_iter.peek().unwrap() < target {
                z_iter.next();
            }
            if *z_iter.peek().unwrap() != target {
                all_equal = false;
            }
        }
        if all_equal {
            return target;
        }
    }
}

aoc::main!();
