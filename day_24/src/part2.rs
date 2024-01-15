use z3::ast::{Ast, Int, Real};

pub fn process(data: &str) -> usize {
    let hailstones = parse(data);
    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Real::new_const(&ctx, v));
    let zero = Int::from_i64(&ctx, 0).to_real();
    for (i, &(x, y, z, dx, dy, dz)) in hailstones[..3].iter().enumerate() {
        let [x, y, z, dx, dy, dz] =
            [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v as _).to_real());
        let t = Real::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let res = s
        .get_model()
        .unwrap()
        .eval(&(&fx + &fy + &fz), true)
        .unwrap();
    res.to_string().strip_suffix(".0").unwrap().parse().unwrap()
}

fn parse(data: &str) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    let convert = |s: &str| s.trim().parse::<f64>().unwrap();
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
            (x, y, z, vx, vy, vz)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 47);
    }
}
