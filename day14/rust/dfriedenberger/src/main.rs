
use aoc2021::reader::read_file;
use aoc2021::parser::create_bingo_boards;
use aoc2021::parser::create_polymer;


fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
   
    let (polymer_template,rules) = create_bingo_boards(&data).unwrap();

   
    let cnt = create_polymer(&polymer_template,&rules,10).unwrap();

    println!("star 1 {}",cnt);
    //end::star1[]

    //tag::star2[]
    let cnt = create_polymer(&polymer_template,&rules,40).unwrap();
    println!("star 2 {}",cnt);
    //end::star2[]

}