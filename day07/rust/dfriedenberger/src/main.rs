
use aoc2021::reader::read_file;

use aoc2021::parser::create_swarm;
use aoc2021::parser::calculate_fuel;
use aoc2021::parser::gaussian_sum;
use aoc2021::parser::linear;



fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let swarm = create_swarm(&data).unwrap();

    let mut min = 0;
    for h in 0..2000 {
        let cnt = calculate_fuel(&swarm,h,linear);

        if min == 0 || cnt < min {
            min = cnt;
        }
    }
    println!("star 1 {}",min);
    //end::star1[]

    //tag::star2[]
    let mut min = 0;
    for h in 0..2000 {
        let cnt = calculate_fuel(&swarm,h,gaussian_sum);

        if min == 0 || cnt < min {
            min = cnt;
        }
    }
    println!("star 2 {}",min);
    //end::star2[]

}