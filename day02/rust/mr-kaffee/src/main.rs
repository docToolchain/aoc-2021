use mr_kaffee_2021_02::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let instant_1 = Instant::now();
    let content = &read_input();
    let sol_1 = calc_position(content);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1.0 * sol_1.1);
    assert_eq!(2120749, sol_1.0 * sol_1.1);

    // solve part 2
    let instant_2 = Instant::now();
    let sol_2 = calc_position_with_aim(content);
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2.0 * sol_2.1);
    assert_eq!(2138382217, sol_2.0 * sol_2.1);
}
