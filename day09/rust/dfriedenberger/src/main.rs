
use aoc2021::reader::read_file;
use aoc2021::parser::create_heightmap;


fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let heightmap = create_heightmap(&data).unwrap();

    println!("star 1 {}",heightmap.cnt_risk_levels());
    //end::star1[]

    //tag::star2[]
    println!("star 2 {}",heightmap.largest_basin_sizes());
    //end::star2[]

}