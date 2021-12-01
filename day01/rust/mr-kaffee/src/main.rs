use mr_kaffee_2021_01::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let content = parse(&read_input());
    let instant_1 = Instant::now();
    let sol_1 = count_increase(&content);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(1374, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let sol_2 = count_increase(&sliding_sums(&content));
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(1418, sol_2);
}
