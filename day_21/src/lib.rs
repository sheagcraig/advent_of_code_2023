use aoc::Point;

pub fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn in_bounds(grid: &Vec<Vec<char>>, point: Point) -> bool {
    point.x >= 0 && point.y >= 0 && point.y < grid.len() as isize && point.x < grid[point.y as usize].len() as isize
}
