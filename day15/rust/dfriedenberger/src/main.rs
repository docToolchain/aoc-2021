
use aoc2021::reader::read_file;
use aoc2021::parser::create_bingo_boards;
use aoc2021::parser::dijeska;
use aoc2021::parser::enlarge;


fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let cave = create_bingo_boards(&data).unwrap();

    let cnt = dijeska(&cave);
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let cave2 = enlarge(&cave).unwrap();
    let cnt = dijeska(&cave2);
    println!("star 2 {}",cnt);
    //end::star2[]

}