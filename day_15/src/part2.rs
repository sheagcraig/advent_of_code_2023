use std::collections::BTreeMap;

pub fn process(data: &str) -> usize {
    let instructions: Vec<_> = data.split(',')
        .map(|s| {
            let op_idx = s.find(['-', '=']).unwrap();
            let op: String = s[op_idx..].chars().take(1).collect();
            let lens_lbl = &s[..op_idx];
            let box_num = hash(lens_lbl);
            let focal_len: Option<usize> = if op == "=" {
                Some(s[op_idx + 1..].parse::<usize>().unwrap())
            } else {
                None
            };
            (box_num, op, lens_lbl, focal_len)
        })
        .collect();
    // println!("{:?}", instructions);
    let mut boxes: BTreeMap<usize, Vec<(&str, usize)>> = BTreeMap::from_iter((0..256).map(|x| (x, Vec::new())));
    for (box_num, op, lens_lbl, focal_len) in instructions {
        if op == "-" {
            boxes.get_mut(&box_num).unwrap().retain(|&x| x.0 != lens_lbl);
        } else if let Some(i) = boxes.get(&box_num).unwrap().iter().position(|&x| x.0 == lens_lbl) {
            boxes.get_mut(&box_num).unwrap()[i] = (lens_lbl, focal_len.unwrap());
        } else {
            boxes.get_mut(&box_num).unwrap().push((lens_lbl, focal_len.unwrap()));
        }
    }
    // dbg!(&boxes);
    // score them
    boxes.into_iter()
        .flat_map(|(box_num, lenses)|
            lenses.iter()
            .enumerate()
            .map(move |(i, l)| (1 + box_num) * (1 + i) * l.1)
                .collect::<Vec<_>>())
        .sum()
}

fn hash(data: &str) -> usize {
    data.chars()
        .map(|c| c as usize)
        .fold(0, |acc, c| (acc + c) * 17 % 256)
}
#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn test_process() {
        let data = include_str!("sample_data_1.txt");
        assert_eq!(process(data), 145);
    }
}
