
use aoc2021::util::read_file;
use aoc2021::util::create_game;



fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let board = create_game(&data,false).unwrap();
   
    println!("start 1 {}",board.count_crossing());
    //end::star1[]

    //tag::star2[]
    let board = create_game(&data,true).unwrap();
   
    println!("start 1 {}",board.count_crossing());
    //end::star2[]

}