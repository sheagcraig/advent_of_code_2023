mod part1;
mod part2;

fn main() {
    let data = include_str!("data.txt");
    let result = part1::process(data);
    println!("Part 1 Result: {result}");
    let result = part2::process(data);
    println!("Part 2 Result: {result}");
}

