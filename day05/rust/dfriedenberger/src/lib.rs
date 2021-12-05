pub mod util;

#[cfg(test)]
mod tests {
    
    use crate::util::create_game;
 

    #[test]
    fn test_it() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            String::from("0,9 -> 5,9"),
            String::from("8,0 -> 0,8"),
            String::from("9,4 -> 3,4"),
            String::from("2,2 -> 2,1"),
            String::from("7,0 -> 7,4"),
            String::from("6,4 -> 2,0"),
            String::from("0,9 -> 2,9"),
            String::from("3,4 -> 1,4"),
            String::from("0,0 -> 8,8"),
            String::from("5,5 -> 8,2")
        ];
        //end::testdata[] 

       
      

     
        //tag::test1[]
        let board = create_game(&vec1,false).unwrap();
        board.dump(10,10);
        assert_eq!(board.count_crossing(), 5);
        //end::test1[]
        

        

         //tag::test2[]
         let board = create_game(&vec1,true).unwrap();
         board.dump(10,10);
         assert_eq!(board.count_crossing(), 12);
         //end::test2[]

        Ok(())
    }

}