use day_22::Bricks;
use std::collections::{BTreeMap, HashSet};

pub fn process(data: &str) -> usize {
    let mut bricks = day_22::parse(data);
    // Settle bricks initially.
    bricks.sort_by_key(|(b1, b2)| b1.z.min(b2.z));
    let bricks: BTreeMap<
        usize,
        Bricks,
    > = bricks
        .into_iter()
        .enumerate()
        .map(|(i, (b1, b2))| {
            (
                i + 1,
                (
                    b1.z.min(b2.z),
                    b1.z.max(b2.z),
                    b1,
                    b2,
                    HashSet::new(),
                    HashSet::new(),
                ),
            )
        })
        .collect();

    let mut result = 0;
    let (_, bricks) = settle_bricks(bricks);
    for (i, _brick) in bricks.iter() {
        let mut disintegrate_one = bricks.clone();
        disintegrate_one.remove(i);
        for (_id, brick) in disintegrate_one.iter_mut() {
            brick.4.clear();
            brick.5.clear();
        }
        let (fell, _) = settle_bricks(disintegrate_one);
        // println!("Brick {i} removal causes {fell} bricks to fall");
        result += fell;
    }
    result
}

fn settle_bricks(
    mut bricks: BTreeMap<
        usize,
        Bricks,
    >,
) -> (
    usize,
    BTreeMap<
        usize,
        Bricks,
    >,
) {
    let mut fallen_count = 0;
    let mut tops = [[0; 10]; 10];
    // Sneaky... the real data set is unsorted.
    // for i in 0..=bricks.len() {
    let binding = bricks.clone();
    let indices: Vec<_> = binding.keys().collect();
    for i in indices {
        if bricks.get(i).is_none() {
            continue;
        }
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
                tops[y][x] = *i;
            }
        }
        let new_z = nearest_below
            .iter()
            .max_by_key(|(_id, z)| z)
            .unwrap_or(&(0, 0))
            .1
            + 1;
        let d = max_z - min_z;
        if min_z != new_z {
            // println!("Brick {i} fell");
            fallen_count += 1;
        }
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
                bricks.get_mut(&id).unwrap().5.insert(*i);
            }
        }
        bricks.insert(*i, (new_z, new_z + d, b1, b2, supported_by, supports));
    }
    (fallen_count, bricks)
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 7);
    }
}
