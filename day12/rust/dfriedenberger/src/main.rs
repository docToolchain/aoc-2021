
use aoc2021::reader::read_file;
use aoc2021::parser::create_caves;
use aoc2021::parser::find_ways;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let caves = create_caves(&data).unwrap();

    let cnt = find_ways(&caves,1);
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let cnt = find_ways(&caves,2);
    println!("star 2 {}",cnt);
    //end::star2[]

}