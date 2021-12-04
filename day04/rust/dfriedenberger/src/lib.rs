pub mod util;

#[cfg(test)]
mod tests {
    
    use crate::util::create_game;
    use crate::util::play_game_util_first_bingo;
    use crate::util::play_game_util_last_bingo;

    #[test]
    fn test_it() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7"),

        ];
        //end::testdata[] 

        let (numbers,mut boards) = create_game(&vec1).unwrap();

        assert_eq!(numbers.len(), 27);
        assert_eq!(boards.len(), 3);

        let board = &boards[0];
        let n = board.get(0,4);
        assert_eq!(n,0);

        //tag::test1[]
        let (unmarked_numbers,called_number) = play_game_util_first_bingo(&numbers,&mut boards).unwrap();
        assert_eq!(unmarked_numbers,188);
        assert_eq!(called_number,24);
        assert_eq!(unmarked_numbers * called_number,4512);
        //end::test1[]
        

         let (numbers,mut boards) = create_game(&vec1).unwrap();

         //tag::test2[]
         let (unmarked_numbers,called_number) = play_game_util_last_bingo(&numbers,&mut boards).unwrap();
         assert_eq!(unmarked_numbers,148);
         assert_eq!(called_number,13);
         assert_eq!(unmarked_numbers * called_number,1924);
         //end::test2[]

        Ok(())
    }

}