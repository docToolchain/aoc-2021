
use aoc2021::reader::read_file;

use aoc2021::parser::create_bingo_boards;
use aoc2021::parser::enhance;

fn main() {

    //tag::star[]
    let data = read_file("input.txt").unwrap();
    let (algorithm,mut image) = create_bingo_boards(&data).unwrap();

    for _i in 0..2 {
        let outside = if image.get_outside() == '.' { '#' } else { '.' };

        image = enhance(&algorithm,&image,outside);
       
    }

    println!("star 1 {}",image.pixels());
    //end::star1[]

    //tag::star2[]
    for _i in 0..48 {
        let outside = if image.get_outside() == '.' { '#' } else { '.' };

        image = enhance(&algorithm,&image,outside);
       
    }
    
    println!("star 2 {}",image.pixels());
    //end::star2[]

}