pub mod reader;
pub mod parser;

#[cfg(test)]
mod tests {
    
    use crate::parser::validate;
    use crate::parser::autocomplete;


    #[test]
    fn test_create_bingo_boards() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 

        
        //tag::test1[]
        let err = validate(&vec2).unwrap();
        assert_eq!(err, 26397);
        //end::test1[]
        

         //tag::test2[]
         let score = autocomplete(&vec2).unwrap();
         assert_eq!(score, 288957);
         //end::test2[]

        Ok(())
    }

}