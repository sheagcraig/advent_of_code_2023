use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point{x: -self.x, y: -self.y}
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {

    // This isn't necessarily "correct"; it's just comparing x, falling back to y values.
    // Arguably, it should probably be distance from origin? As of right now I'm only using it
    // to allow me to put points in a tuple in a BinHeap where I don't care about the ordering
    // of the points (they're not the first item in the tuple). Can revisit and fix in the future
    // if needed.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl Point {
    pub fn distance(self, rhs: Self) -> usize {
        ((self.x - rhs.x).abs() + (self.y - rhs.y).abs()) as usize
    }

    pub fn neighbors(self) -> Vec<Point> {
        vec![self + UP, self + DOWN, self + LEFT, self + RIGHT]
    }
}

pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };
pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };