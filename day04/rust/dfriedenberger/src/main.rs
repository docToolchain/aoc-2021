
use aoc2021::util::read_file;
use aoc2021::util::create_game;
use aoc2021::util::play_game_util_first_bingo;
use aoc2021::util::play_game_util_last_bingo;


fn main() {

    //tag::star1[]
    let data = read_file("input.txt").unwrap();
    let (numbers,mut boards) = create_game(&data).unwrap();
    let (unmarked_numbers,called_number) = play_game_util_first_bingo(&numbers,&mut boards).unwrap();
    println!("start 1 {}",unmarked_numbers * called_number);
    //end::star1[]

    //tag::star2[]
    let (numbers,mut boards) = create_game(&data).unwrap();
    let (unmarked_numbers,called_number) = play_game_util_last_bingo(&numbers,&mut boards).unwrap();
    println!("start 2 {}",unmarked_numbers * called_number);
    //end::star2[]

}