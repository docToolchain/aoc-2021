use mr_kaffee_2021_07::*;
use std::time::Instant;

const INPUT: &str = include_str!("../input.txt");
const EXP_1: usize = 337_833;
const EXP_2: usize = 96_678_050;

fn main() {     
    let start = Instant::now();
    let crabs = parse(INPUT);

    // solve part 1
    let start_1 = Instant::now();
    let sol_1 = solution_1(&crabs);
    println!("Solved part 1 in {:?}: {:?}", start_1.elapsed(), sol_1);
    assert_eq!(EXP_1, sol_1);

    // solve part 2
    let start_2 = Instant::now();
    let sol_2 = solution_2(&crabs);
    println!("Solved part 2 in {:?}: {:?}", start_2.elapsed(), sol_2);
    assert_eq!(EXP_2, sol_2);

    println!("Solved puzzle in {:?}", start.elapsed());
}
