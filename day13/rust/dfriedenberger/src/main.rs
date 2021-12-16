
use aoc2021::reader::read_file;
use aoc2021::parser::create_bingo_boards;
use aoc2021::parser::fold;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
    let (mut paper,instructions) = create_bingo_boards(&data).unwrap();

    let cnt = fold(&mut paper,&instructions,true);
    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let (mut paper,instructions) = create_bingo_boards(&data).unwrap();
    let cnt = fold(&mut paper,&instructions,false);
    println!("star 2");
    paper.dump();
    //end::star2[]

}