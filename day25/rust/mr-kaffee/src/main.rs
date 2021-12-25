use mr_kaffee_2021_25::*;
use std::time::Instant;

pub const INPUT: &str = include_str!("../input.txt");
pub const EXP_1: usize = 0;

fn main() {     
    let timer = Instant::now();
    let data = parse(INPUT);

    let timer_1 = Instant::now();
    let sol_1 = solution_1(&data);
    println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
    assert_eq!(EXP_1, sol_1);

    println!("-> Solved day 25 in {:?}\n", timer.elapsed());
}
