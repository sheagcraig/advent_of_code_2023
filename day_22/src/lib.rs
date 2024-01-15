use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3d {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub type Bricks = (
    usize,
    usize,
    Point3d,
    Point3d,
    HashSet<usize>,
    HashSet<usize>,
);

pub fn parse(data: &str) -> Vec<(Point3d, Point3d)> {
    data.lines()
        .map(|line| {
            let split = line.split(['~', ',']).map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
            (Point3d{x: split[0], y: split[1], z: split[2]},
             Point3d{x: split[3], y: split[4], z: split[5]})
        }).collect()
}