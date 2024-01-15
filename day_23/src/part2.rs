use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

type GridPoint = (i32, i32);
type EdgeMap = HashMap<GridPoint, HashSet<(GridPoint, i32)>>;
static VECTORS: [GridPoint; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn process(data: &str) -> usize {
    let (edges, start, end) = parse(data);
    let mut valid_paths: Vec<i32> = vec![];
    let mut stack = VecDeque::from_iter([(start, vec![start], 0)]);
    while !stack.is_empty() {
        let (current, path_taken, path_length) = stack.pop_back().unwrap();

        if current == end {
            valid_paths.push(path_length);
            continue;
        }

        for (next_vertex, distance) in edges.get(&current).unwrap() {
            if !path_taken.contains(next_vertex) {
                let mut next_path = path_taken.clone();
                next_path.push(*next_vertex);
                stack.push_back((*next_vertex, next_path, path_length + distance));
            }
        }
    }
    *valid_paths.iter().max().unwrap() as usize
}

// 1, 0 -> 3, 5
// 3, 5 -> 5, 13
// 5, 13 -> 13, 19
// 13, 19 -> 13, 13
// 13, 13 -> 11, 3
// 21, 11 -> 19, 19

/// Return a set of nodes and a map of nodes to edges
fn parse(
    data: &str,
) -> (
    EdgeMap,
    GridPoint,
    GridPoint,
) {
    // Build a set of nodes from the input which have more than 2 neighbors.
    // 1 neighbor means dead end
    // 2 neighbors means the node can be culled because it's the previous node and one next node.
    // 3-4 neighbors means the node is a junction.

    // But first, break down the data into a vec of vec of chars, because we'll need to look at whether
    // candidate neighbors are walls or not.
    let grid: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    // Build the set of nodes we care about.
    let mut nodes = (0..(grid.len()))
        .cartesian_product(0..(grid[0].len()))
        .filter_map(|(y, x)| {
            if grid[y][x] == '#' {
                return None;
            }
            let mut neighbors = vec![];
            for dir in VECTORS.iter() {
                let next = (x as i32 + dir.0, y as i32 + dir.1);
                if next.0 >= 0
                    && next.1 >= 0
                    && next.0 < grid[0].len() as i32
                    && next.1 < grid.len() as i32
                    && grid[next.1 as usize][next.0 as usize] != '#'
                {
                    neighbors.push(next);
                }
            }
            if neighbors.len() > 2 {
                Some((x as i32, y as i32))
            } else {
                None
            }
        })
        .collect::<HashSet<GridPoint>>();

    // Add in start and end nodes.
    let start = grid
        .first()
        .unwrap()
        .iter()
        .find_position(|&c| c == &'.')
        .map(|(x, _)| (x as i32, 0))
        .unwrap();
    let end = grid
        .last()
        .unwrap()
        .iter()
        .find_position(|&c| c == &'.')
        .map(|(x, _)| (x as i32, grid.len() as i32 - 1))
        .unwrap();
    nodes.extend([start, end]);

    let mut edges = HashMap::new();

    for node in nodes.iter().sorted() {
        let mut q = VecDeque::from_iter([(*node, 0)]);
        let mut explored: HashSet<GridPoint> = HashSet::new();

        while !q.is_empty() {
            let ((x, y), distance) = q.pop_front().unwrap();
            if !explored.insert((x, y)) {
                continue;
            }
            for dir in VECTORS {
                let next = (x + dir.0, y + dir.1);
                // Test next point is in bounds and not a rock
                if next.0 >= 0
                    && next.1 >= 0
                    && next.0 < grid[0].len() as i32
                    && next.1 < grid.len() as i32
                    && grid[next.1 as usize][next.0 as usize] != '#'
                {
                    if nodes.contains(&next) && next != *node {
                        edges
                            .entry(*node)
                            .or_insert_with(HashSet::new)
                            .insert((next, distance + 1));
                        continue;
                    }

                    q.push_back((next, distance + 1));
                }
            }
        }
    }

    (edges, start, end)
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 154);
    }
}
