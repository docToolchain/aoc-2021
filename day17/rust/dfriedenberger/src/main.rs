

use aoc2021::parser::style_shot;
use aoc2021::parser::within_target;


fn main() {

    //tag::star[]
    //target area: x=25..67, y=-260..-200

    let max_height = style_shot(25,67,-260,-200).unwrap();
    println!("star 1 {}",max_height);
    //end::star1[]

    //=> 7 259 flew 33670
    //tag::star2[]
    let cnt = within_target(25,67,-260,-200).unwrap();
    println!("star 2 {}",cnt);
    //end::star2[]

}