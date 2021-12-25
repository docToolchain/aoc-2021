use std::env;

use mr_kaffee_2021_00::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut from = 1;
    let mut to = 25;

    let mut k = 1;
    while k < args.len() {
        match args[k].as_str() {
            "--from" => {
                from = args[k + 1].parse().expect("Could not parse from");
                k += 2;
            }
            "--to" => {
                to = args[k + 1].parse().expect("Could not parse to");
                k += 2
            }
            _ => panic!("Illegal argument: {}", args[k]),
        }
    }

    solve(from..=to);
}
