use mr_kaffee_2021_11::*;
use std::time::Instant;

fn main() {     
    let start = Instant::now();
    let data = parse(include_str!("../input.txt"));

    // solve part 1
    let start_1 = Instant::now();
    let sol_1 = solution_1(&data);
    println!("Solved part 1 in {:?}: {:?}", start_1.elapsed(), sol_1);
    assert_eq!(1_652, sol_1);

    // solve part 2
    let start_2 = Instant::now();
    let sol_2 = solution_2(&data);
    println!("Solved part 2 in {:?}: {:?}", start_2.elapsed(), sol_2);
    assert_eq!(220, sol_2);

    println!("Total: {:?}", start.elapsed());
}
