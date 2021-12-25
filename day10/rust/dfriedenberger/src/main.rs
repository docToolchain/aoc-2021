
use aoc2021::reader::read_file;
use aoc2021::parser::validate;
use aoc2021::parser::autocomplete;



fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let err = validate(&data).unwrap();
    println!("star 1 {}",err);
    //end::star1[]

    //tag::star2[]
    let score = autocomplete(&data).unwrap();
    println!("star 2 {}",score);
    //end::star2[]

}