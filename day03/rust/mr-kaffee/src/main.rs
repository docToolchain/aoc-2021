use mr_kaffee_2021_03::*;
use std::time::Instant;

fn main() {  
    let timer = Instant::now();
    let (vals, len) = parse(include_str!("../input.txt"));

    // solve part 1
    let timer_1 = Instant::now();
    let (_, _, sol_1) = calc_gamma_epsilon(&vals, len);
    println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
    assert_eq!(4_006_064, sol_1);

    // solve part 2
    let timer_2 = Instant::now();
    let (_, _, sol_2) = calc_ratings(&vals, len);
    println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
    assert_eq!(5_941_884, sol_2);

    println!("Solved puzzle in {:?}", timer.elapsed());
}
