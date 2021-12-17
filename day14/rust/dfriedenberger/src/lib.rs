pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::fields::BingoBoard;
    use crate::fields::Diagram;
    use crate::parser::create_bingo_boards;
    use crate::parser::create_polymer;
    
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
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"

        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let (polymer_template,rules) = create_bingo_boards(&vec2).unwrap();

        assert_eq!(rules.len(), 16);
        //end::test1[]
        
        let cnt = create_polymer(&polymer_template,&rules,10).unwrap();
        assert_eq!(cnt, 1588);
        

         //tag::test2[]
         
         let cnt = create_polymer(&polymer_template,&rules,40).unwrap();
         assert_eq!(cnt, 2188189693529);
         //end::test2[]

        Ok(())
    }

}