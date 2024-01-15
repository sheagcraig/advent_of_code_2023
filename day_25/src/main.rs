mod part1;

fn main() {
    let data = include_str!("data.txt");
    let result = part1::process(data);
    println!("Part 1 Result: {result}");
}

