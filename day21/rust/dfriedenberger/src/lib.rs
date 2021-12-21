pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::parser::play;
    use crate::parser::play2;

   


    #[test]
    fn test_play() -> Result<(), String> {

      
        //tag::test1[]

        //end::test1[]
        
        let cnt = play(&mut 4,&mut 8);
        assert_eq!(cnt, 739785);


         //tag::test2[]

         //end::test2[]

        Ok(())
    }

    #[test]
    fn test_play2() -> Result<(), String> {

      
        //tag::test1[]

        //end::test1[]
        
        let (p1 , p2) = play2(4,8);
        assert_eq!(p1, 444356092776315); 
        assert_eq!(p2, 341960390180808);


         //tag::test2[]

         //end::test2[]

        Ok(())
    }

}