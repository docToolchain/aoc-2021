pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::fields::BingoBoard;
    use crate::fields::Diagram;
    use crate::parser::create_bingo_boards;
    use crate::parser::dijeska;
    use crate::parser::enlarge;

    #[test]
    fn test_bingoboard() -> Result<(), String> {

        let board = BingoBoard::new(vec![vec![ 0 , 1 , 2 ] ,vec![ 5 , 8 , 9 ],vec![ 10 , 17 , 12 ] ]);
        assert_eq!(board.get(0,2), 2);
        Ok(())

    }

    #[test]
    fn test_diagram() -> Result<(), String> {

        let mut diagram = Diagram::new();
        diagram.set(1,2);
        diagram.dump(4,3);
        assert_eq!(1, 1);
        Ok(())

    }

    #[test]
    fn test_create_bingo_boards() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let cave = create_bingo_boards(&vec2).unwrap();

        let cnt = dijeska(&cave);
        assert_eq!(cnt, 40);
        //end::test1[]
        

        

         //tag::test2[]
        let cave2 = enlarge(&cave).unwrap();
        let cnt = dijeska(&cave2);
        assert_eq!(cnt, 315);
         //end::test2[]

        Ok(())
    }

}