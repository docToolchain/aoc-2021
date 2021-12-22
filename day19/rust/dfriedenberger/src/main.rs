
use aoc2021::reader::read_file;

use aoc2021::parser::create_scanners;
use aoc2021::parser::join_all;
use aoc2021::parser::manhattan;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let mut scanners = create_scanners(&data).unwrap();

    let mut centers = Vec::new();
    centers.push((0,0,0));
    let cnt = join_all(&mut scanners,&mut centers);
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let cnt = manhattan(&centers);

    println!("star 2 {}",cnt);
    //end::star2[]

}