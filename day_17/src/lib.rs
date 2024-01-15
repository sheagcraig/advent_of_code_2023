use aoc::{DOWN, LEFT, Point, RIGHT, UP};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub fn parse(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

pub fn print_path(grid: &[Vec<u32>], path: &[(isize, isize)]) {
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if path.contains(&(x as isize, y as isize)) {
                print!("*");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
}

pub fn in_bounds(p: Point, grid: &Vec<Vec<u32>>) -> bool {
    p.x < grid[0].len() as isize && p.y < grid.len() as isize && p.x >= 0 && p.y >= 0
}
pub fn in_bounds_old(p: (isize, isize), grid: &Vec<Vec<u32>>) -> bool {
    p.0 < grid[0].len() as isize && p.1 < grid.len() as isize && p.0 >= 0 && p.1 >= 0
}

pub fn djikstra_min_heap(grid: &Vec<Vec<u32>>, min_steps: isize, max_steps: isize) -> i64 {
    let mut costs: HashMap<(Point, Point), i64> = HashMap::new();
    let mut q = BinaryHeap::from_iter([(Reverse(0), Point{x: 0, y:0}, Point{x: 0, y: 0})]);
    let end = Point{x: grid[0].len() as isize -1, y: grid.len() as isize -1};

    while let Some((cost, current_pt, delta)) = q.pop() {
        if current_pt == end {
            return cost.0
        }
        if costs.get(&(current_pt, delta)).is_some_and(|&c| cost > Reverse(c)) {
            continue
        }
        let neighbors = get_neighbors(grid, current_pt, delta, cost.0, min_steps, max_steps);
        for (p, d, c) in neighbors {
            let new_cost = c;
            if new_cost < *costs.get(&(p, d)).unwrap_or(&i64::MAX) {
                costs.insert((p, d), new_cost);
                q.push((Reverse(new_cost), p, d))
            }
        }
    }
    i64::MAX
}

fn get_neighbors(grid: &Vec<Vec<u32>>, current_pt: Point, delta: Point, cost: i64, min_steps: isize, max_steps: isize) -> Vec<(Point, Point, i64)> {
    [LEFT, RIGHT, UP, DOWN].iter()
        .flat_map(|&next_dir| {
            let mut results = Vec::new();
            // Filter out next points which are moving forward or are going back the way we came;
            // we've already figured out the full forward neighbors for a point when we get here, and
            // the crucible can only move L, R, or forward (so no back).
            if delta == next_dir || delta == -next_dir {
                return results
            }
            // Calculate neighbors up to 3 steps away.
            let mut new_cost = cost;
            for steps in 1..=max_steps {
                let next = current_pt + Point{x: steps, y: steps} * next_dir;
                if in_bounds(next, grid) {
                    new_cost += grid[next.y as usize][next.x as usize] as i64;
                    if steps < min_steps { continue }
                    results.push((next, next_dir, new_cost))
                }
            }
            results
        })
        .collect()
}
