use std::collections::{BTreeMap, HashSet};
use day_22::Bricks;

pub fn process(data: &str) -> usize {
    let mut bricks =  day_22::parse(data);
    // Track the highest brick so far for each position.
    let mut tops = [[0;10];10];
    // Sneaky... the real data set is unsorted.
    bricks.sort_by_key(|(b1, b2)| b1.z.min(b2.z));
    let mut bricks: BTreeMap<usize, Bricks> = bricks.into_iter().enumerate()
        .map(|(i, (b1, b2))| (i+1, (b1.z.min(b2.z), b1.z.max(b2.z), b1, b2, HashSet::new(), HashSet::new())))
        .collect();
    for i in 1..=bricks.len() {
        let (min_z, max_z, mut b1, mut b2, mut supported_by, supports) = bricks[&i].clone();
        let mut nearest_below = Vec::new();
        for x in b1.x..=b2.x {
            #[allow(clippy::needless_range_loop)]
            for y in b1.y..=b2.y {
                let brick_below_id = tops[y][x];
                if brick_below_id > 0 {
                    // We found a brick below this one.
                    // Add this brick to the list of bricks supported by the brick below.
                    nearest_below.push((brick_below_id, bricks[&brick_below_id].1));
                }
                tops[y][x] = i;
            }
        }
        let new_z = nearest_below.iter().max_by_key(|(_id, z)| z).unwrap_or(&(0, 0)).1 + 1;
        let d = max_z - min_z;
        if b1.z < b2.z {
            b1.z = new_z;
            b2.z = b1.z + d;
        } else {
            b2.z = new_z;
            b1.z = b2.z + d;
        }
        for (id, z) in nearest_below {
            if z == new_z - 1 {
                supported_by.insert(id);
                bricks.get_mut(&id).unwrap().5.insert(i);
            }
        }
        bricks.insert(i, (new_z, new_z + d, b1, b2, supported_by, supports));
    }
    // dbg!(&bricks);
    let mut result = 0;
    for (i, brick) in bricks.iter() {
        if i == &1 || i == &13 {
            dbg!(&brick);
        }
        if brick.5.iter().all(|id| bricks[id].4.len() != 1) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 5);
    }
}
