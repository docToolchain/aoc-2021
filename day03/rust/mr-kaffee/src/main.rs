use mr_kaffee_2021_03::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {     
    // solve part 1
    let instant_1 = Instant::now();
    let (vals, len) = parse(&read_input());
    let (_, _, sol_1) = calc_gamma_epsilon(&vals, len);
    println!("Solved part 1 in {:?}: {:?}", instant_1.elapsed(), sol_1);
    assert_eq!(4_006_064, sol_1);

    // solve part 2
    let instant_2 = Instant::now();
    let (_, _, sol_2) = calc_ratings(&vals, len);
    println!("Solved part 2 in {:?}: {:?}", instant_2.elapsed(), sol_2);
    assert_eq!(5_941_884, sol_2);
}
