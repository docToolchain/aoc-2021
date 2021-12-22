
use aoc2021::reader::read_file;
use aoc2021::parser::calculate;
use aoc2021::parser::calculate_max;



fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    
    let magnitude = calculate(&data);
    println!("star 1 {}",magnitude);
    //end::star1[]

    //tag::star2[]
    let max_magnitude = calculate_max(&data);

    println!("star 2 {}",max_magnitude);
    //end::star2[]

}