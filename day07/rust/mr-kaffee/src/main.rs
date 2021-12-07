use mr_kaffee_2021_07::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let instant_1 = Instant::now();
    let crabs = parse(&read_input());
    let sol_1 = get_optimal_positions_fuel(&crabs, COST_LINEAR);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(337_833, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let sol_2 = get_optimal_positions_fuel(&crabs, COST_INCREASING);
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(96_678_050, sol_2);
}
