use mr_kaffee_2021_04::*;
use std::{fs, time::Instant};

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read 'input.txt'")
}

fn main() {
    // solve part 1 & 2
    let instant_1 = Instant::now();
    let (mut boards, draws) = parse(&read_input());
    let (sol_1, sol_2) = get_scores(&mut boards, &draws);
    println!(
        "Solved part 1 & 2 in {:?}: {:?} / {:?}",
        instant_1.elapsed(),
        sol_1,
        sol_2
    );
    assert_eq!(10_680, sol_1);
    assert_eq!(31_892, sol_2);
}
