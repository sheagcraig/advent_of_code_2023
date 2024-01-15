use day_20::parse;

pub fn process(data: &str) -> usize {
    let setup =  parse(data);
    let (_low_count, _high_count, cycles) = day_20::solve(setup);
    cycles.values().map(|o| o.unwrap()).product()
}
