
use aoc2021::util::read_file;
use aoc2021::util::calculate_increases;
use aoc2021::util::calculate_window;

fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    println!("star1 {}",calculate_increases(&data).unwrap());
    //end::star1[]

    //tag::star2[]
    let data2 = calculate_window(&data).unwrap();
    println!("star2 {}",calculate_increases(&data2).unwrap());
    //end::star2[]

}