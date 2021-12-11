use mr_kaffee_2021_04::*;
use std::time::Instant;

fn main() {
    // solve part 1 & 2
    let timer = Instant::now();
    let (mut boards, draws) = parse(include_str!("../input.txt"));
    let (sol_1, sol_2) = play(&mut boards, &draws);
    println!(
        "Solved part 1 & 2 in {:?}: {:?} / {:?}",
        timer.elapsed(),
        sol_1,
        sol_2
    );
    assert_eq!(10_680, sol_1);
    assert_eq!(31_892, sol_2);
}
