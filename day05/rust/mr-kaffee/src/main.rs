use mr_kaffee_2021_05::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let instant_1 = Instant::now();
    let sol_1 = do_something(&read_input());
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(0, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let sol_2 = 0;
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(0, sol_2);
}
