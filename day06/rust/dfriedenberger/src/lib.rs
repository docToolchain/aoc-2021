pub mod reader;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::parser::create_swarm;
    use crate::parser::incr_and_count;
    

    #[test]
    fn test_swarm() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "3,4,3,1,2"
        ];
        //end::testdata[] 

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();

        

     
        //tag::test1[]
        let mut swarm = create_swarm(&vec2).unwrap();
        let cnt = incr_and_count(&mut swarm,18);
        assert_eq!(cnt, 26);

        let cnt = incr_and_count(&mut swarm,80 - 18);
        assert_eq!(cnt, 5934);
        //end::test1[]
        

        

         //tag::test2[]
         let cnt = incr_and_count(&mut swarm,256 - 80);
         assert_eq!(cnt, 26984457539);
         //end::test2[]

        Ok(())
    }

}