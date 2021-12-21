pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 

    use crate::parser::reduce;
    use crate::parser::add;
    use crate::parser::calculate;
    use crate::parser::calculate_max;
    use crate::parser::magnitude;

    #[test]
    fn test_reduce() -> Result<(), String> {

        let result = reduce(&String::from("[[[[[9,8],1],2],3],4]"));
        assert_eq!(result, String::from("[[[[0,9],2],3],4]"));
        Ok(())

    }

    #[test]
    fn test_reduce2() -> Result<(), String> {

        let result = reduce(&String::from("[[6,[5,[4,[3,2]]]],1]"));
        assert_eq!(result, String::from("[[6,[5,[7,0]]],3]"));
        Ok(())

    }


    #[test]
    fn test_reduce3() -> Result<(), String> {

        let result = reduce(&String::from("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"));
        assert_eq!(result, String::from("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
        Ok(())

    }

    #[test]
    fn test_reduce4() -> Result<(), String> {

        let result = reduce(&String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
        assert_eq!(result, String::from("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        Ok(())

    }

    #[test]
    fn test_add() -> Result<(), String> {

        let result = add(&String::from("[[[[4,3],4],4],[7,[[8,4],9]]]"),&String::from("[1,1]"));
        assert_eq!(result, String::from("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"));
        Ok(())

    }
    
    #[test]
    fn test_magnitude() -> Result<(), String> {

        let result = magnitude(&String::from("[[9,1],[1,9]]"));
        assert_eq!(result, 129);
        Ok(())

    }
    

    


    #[test]
    fn test_create_bingo_boards() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]"
        ];

        let data : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        
        let result = calculate(&data);
        assert_eq!(result, 3488);

        
        //end::test1[]
        

        

         //tag::test2[]
         //assert_eq!(boards.len(), 3);

         //end::test2[]

        Ok(())
    }

    #[test]
    fn test_max_magnitude() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ];

        let data : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        
        let result = calculate_max(&data);
        assert_eq!(result, 3993);

        
        //end::test1[]
        

        

         //tag::test2[]
         //assert_eq!(boards.len(), 3);

         //end::test2[]

        Ok(())
    }


}