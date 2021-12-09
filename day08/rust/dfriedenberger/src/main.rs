
use aoc2021::reader::read_file;
use aoc2021::parser::parse_entries;
use aoc2021::parser::count_digits1478;
use aoc2021::parser::determine;



fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let entries = parse_entries(&data).unwrap();

    let cnt = count_digits1478(&entries);
    println!("start 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let all = determine(&entries);
    println!("start 2 {}",all);
    //end::star2[]

}