
use aoc2021::reader::read_file;

use aoc2021::parser::create_swarm;
use aoc2021::parser::incr_and_count;
    


fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let mut swarm = create_swarm(&data).unwrap();
    let cnt = incr_and_count(&mut swarm,80);
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let cnt = incr_and_count(&mut swarm,256-80);
    println!("star 2 {}",cnt);
    //end::star2[]

}