use itertools::Itertools;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub fn process(data: &str, range: RangeInclusive<f64>) -> usize {
    let hailstones = parse(data);
    // calculate slope intercept form of line given by hail stone point and change in x and y
    hailstones
        .iter()
        .map(|(p, v)| {
            let m = v.y as f64 / v.x as f64;
            let b = p.y as f64 - (p.x as f64 * m);
            (m, b, p, v)
        })
        .combinations(2)
        .filter_map(|v| {
            let (m1, b1, p1, d1) = v[0];
            let (m2, b2, p2, d2) = v[1];
            let x = (b1 - b2) / (m2 - m1);
            let y = m1 * x + b1;
            if range.contains(&x)
                && range.contains(&y)
                && !(x.is_infinite() || y.is_infinite())
                && future(p1, d1, x, y)
                && future(p2, d2, x, y)
            {
                Some((x, y))
            } else {
                None
            }
        })
        .count()
}

fn future(p: &Point3d, d: &Point3d, x: f64, y: f64) -> bool {
    let x_future = if d.x > 0 {
        x > p.x as f64
    } else {
        x < p.x as f64
    };
    let y_future = if d.y > 0 {
        y > p.y as f64
    } else {
        y < p.y as f64
    };
    x_future && y_future
}

fn parse(data: &str) -> Vec<(Point3d, Point3d)> {
    let convert = |s: &str| s.trim().parse::<isize>().unwrap();
    data.lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let mut pt_coords = p.split(", ");
            let (x, y, z) = (
                convert(pt_coords.next().unwrap()),
                convert(pt_coords.next().unwrap()),
                convert(pt_coords.next().unwrap()),
            );
            let mut v_coords = v.split(", ");
            let (vx, vy, vz) = (
                convert(v_coords.next().unwrap()),
                convert(v_coords.next().unwrap()),
                convert(v_coords.next().unwrap()),
            );
            (
                Point3d { x, y, z },
                Point3d {
                    x: vx,
                    y: vy,
                    z: vz,
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data, 7.0..=27.0), 2);
    }
}
