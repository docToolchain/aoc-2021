
use aoc2021::util::read_file;
use aoc2021::util::parse;
use aoc2021::util::course;
use aoc2021::util::course2;


fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let parsed = parse(&data).unwrap();
    let (position,deep) = course(&parsed).unwrap();
    println!("star 1 {}",position * deep);
    //end::star1[]

    //tag::star2[]
    let (position,deep) = course2(&parsed).unwrap();
    println!("star 2 {}",position * deep);

    //end::star2[]

}