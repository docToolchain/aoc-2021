
use aoc2021::reader::read_file;

use aoc2021::parser::parse_hex;
use aoc2021::parser::add_versions;
use aoc2021::parser::calc_value;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
    let bits = parse_hex(&data).unwrap();

    let sum = add_versions(&bits).unwrap();
    println!("star 1 {}",sum);
    //end::star1[]

    //tag::star2[]
    let val = calc_value(&bits).unwrap();
    println!("star 2 {}",val);
    //end::star2[]

}