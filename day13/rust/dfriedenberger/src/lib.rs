pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::fields::BingoBoard;
    use crate::fields::Diagram;
    use crate::parser::create_bingo_boards;
    use crate::parser::fold;

    #[test]
    fn test_bingoboard() -> Result<(), String> {

        let board = BingoBoard::new(vec![vec![ 0 , 1 , 2 ] ,vec![ 5 , 8 , 9 ],vec![ 10 , 17 , 12 ] ]);
        assert_eq!(board.get(0,2), 2);
        Ok(())

    }


    #[test]
    fn test_create_bingo_boards() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let (mut paper,instructions) = create_bingo_boards(&vec2).unwrap();

        assert_eq!(instructions.len(), 2);

        let cnt = fold(&mut paper,&instructions,true);
        paper.dump();
        assert_eq!(cnt, 17);

        //end::test1[]
        

        

         //tag::test2[]

         //end::test2[]

        Ok(())
    }

}