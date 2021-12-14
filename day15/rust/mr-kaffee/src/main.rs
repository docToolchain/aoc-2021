use mr_kaffee_2021_15::*;
use std::time::Instant;

/// puzzle input
pub const INPUT: &str = include_str!("../input.txt");
/// expected result part 1
pub const EXP_1: usize = 0;
/// expected result part 2
pub const EXP_2: usize = 1;

fn main() {     
    let start = Instant::now();
    let data = parse(INPUT);

    // solve part 1
    let start_1 = Instant::now();
    let sol_1 = solution_1(&data);
    println!("Solved part 1 in {:?}: {:?}", start_1.elapsed(), sol_1);
    assert_eq!(EXP_1, sol_1);

    // solve part 2
    let start_2 = Instant::now();
    let sol_2 = solution_2(&data);
    println!("Solved part 2 in {:?}: {:?}", start_2.elapsed(), sol_2);
    assert_eq!(EXP_2, sol_2);

    println!("Solved puzzle in {:?}", start.elapsed());
}
