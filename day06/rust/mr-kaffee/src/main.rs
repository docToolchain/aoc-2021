use mr_kaffee_2021_06::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let instant_1 = Instant::now();
    let fishes = parse(&read_input());
    let sol_1 = simulate_and_count(fishes, 80);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(376_194, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let fishes = parse(&read_input());
    let sol_2 = simulate_and_count(fishes, 256);
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(1_693_022_481_538, sol_2);
}
