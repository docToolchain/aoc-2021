
use aoc2021::util::read_file;
use aoc2021::util::count_bits;
use aoc2021::util::power_consumption;
use aoc2021::util::filter;



fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let bits = count_bits(&data).unwrap();
    let (gamma , epsilon) = power_consumption(&bits,data.len()).unwrap();
    println!("start 1 {}",gamma * epsilon);
    //end::star1[]

    //tag::star2[]
    let (oxygen_generator_rating , co2_scrubber_rating) = filter(&data).unwrap();
    println!("start 2 {}",oxygen_generator_rating * co2_scrubber_rating);
    //end::star2[]

}