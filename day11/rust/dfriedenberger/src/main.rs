
use aoc2021::reader::read_file;

use aoc2021::parser::create_octopus_grid;
use aoc2021::parser::simulate_steps;
use aoc2021::parser::simulate_until_superflash;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let mut grid = create_octopus_grid(&data).unwrap();
    let flashes = simulate_steps(&mut grid,100);

    println!("star 1 {}",flashes);
    //end::star1[]

    //tag::star2[]
    let mut grid = create_octopus_grid(&data).unwrap();
    let steps = simulate_until_superflash(&mut grid);

    println!("star 2 {}",steps);
    //end::star2[]

}