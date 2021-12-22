
use aoc2021::parser::play;
use aoc2021::parser::play2;
use std::cmp;

fn main() {

    //tag::star[]
    //Player 1 starting position: 8
    //Player 2 starting position: 4
    let cnt = play(&mut 8,&mut 4);   
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let (p1 , p2) = play2(8,4);

    println!("star 2 {}",cmp::max(p1,p2));
    //end::star2[]

}