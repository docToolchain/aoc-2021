use mr_kaffee_2021_01::*;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let content = parse(include_str!("../input.txt"));

    // solve part 1
    let timer_1 = Instant::now();
    let sol_1 = count_increase_with_offset(&content, 1);
    println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
    assert_eq!(1_374, sol_1);

    // solve part 2
    let timer_2 = Instant::now();
    let sol_2 = count_increase_with_offset(&content, 3);
    println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
    assert_eq!(1_418, sol_2);

    println!("Solved puzzle in {:?}", timer.elapsed());
}
