use mr_kaffee_2021_00::*;
use std::time::Instant;
use std::fs;

//tag::read_input[]
fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Could not read from file")
}
//end::read_input[]

//tag::main[]
fn main() {
    let instant = Instant::now();
    write_output(&read_input());
    println!("Done in {:?}", instant.elapsed());
}
//end::main[]
