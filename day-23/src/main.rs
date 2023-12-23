use std::collections::{HashSet, HashMap};

type Cell = (i32, i32);
type Grid = HashMap<Cell, char>;
type Graph = HashMap<Cell, Vec<(u32, Cell)>>;

fn parse_grid(grid: &str) -> Grid {
    grid.lines()
        .enumerate()
        .flat_map(|(y, r)| {
            r.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

fn make_graph(grid: &HashMap<Cell, char>, slopes: bool) -> Graph {
    grid.iter()
        .filter(|(_, &t)| t != '#')
        .map(|(&(x, y), &tile)| {
            let neighbours = [
                ((x + 1, y), '>'),
                ((x - 1, y), '<'),
                ((x, y - 1), '^'),
                ((x, y + 1), 'v'),
            ];
            let neighbours = neighbours.iter()
                .filter(|&&(_, s)| !slopes || tile == '.' || tile == s)
                .filter(|(c, _)| *grid.get(c).unwrap_or(&'#') != '#')
                .map(|&(c, _)| (1, c))
                .collect();
            ((x, y), neighbours)
        })
        .collect()
}

fn simplify_node(graph: &mut Graph, source: Cell, between: Cell, (target_dist, target): (u32, Cell)) {
    let connected = graph.get_mut(&source).unwrap();
    let (between_dist, between) = connected.iter_mut()
        .find(|&&mut (_, c)| c == between)
        .unwrap();
    *between_dist += target_dist;
    *between = target;
}

fn simplify_undirected_graph(graph: &mut Graph) {
    let betweens = graph.iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&c, _)| c)
        .collect::<Vec<_>>();
    for between in betweens {
        let nodes = graph.remove(&between).unwrap();
        let (a_dist, a) = nodes[0];
        let (b_dist, b) = nodes[1];
        simplify_node(graph, a, between, (b_dist, b));
        simplify_node(graph, b, between, (a_dist, a));
    }
}

fn longest_path(graph: &Graph, target: Cell, visited: &mut HashSet<Cell>, length: u32, cell: Cell) -> u32 {
    if cell == target { 
        return length;
    }
    if !visited.insert(cell) {
        return 0;
    }
    let neighbours = graph.get(&cell).unwrap(); 
    let longest = neighbours.iter()
        .map(|&(d, c)| longest_path(graph, target, visited, length + d, c))
        .max()
        .unwrap_or_default();
    visited.remove(&cell);
    longest
}

fn part_1(input: String) -> u32 {
    let grid = parse_grid(&input);
    let graph = make_graph(&grid, true);
    let target = *graph.keys().max().unwrap();
    longest_path(&graph, target, &mut HashSet::new(), 0, (1, 0))
}

fn part_2(input: String) -> u32 {
    let grid = parse_grid(&input);
    let mut graph = make_graph(&grid, false);
    simplify_undirected_graph(&mut graph);
    let target = *graph.keys().max().unwrap();
    longest_path(&graph, target, &mut HashSet::new(), 0, (1, 0))
}

aoc::main!();
