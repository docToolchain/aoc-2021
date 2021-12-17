
pub mod parser;

#[cfg(test)]
mod tests {
    
 
 
    use crate::parser::style_shot;
    use crate::parser::within_target;
    
    

    #[test]
    fn test_style_shot() -> Result<(), String> {

       
        //tag::test1[]
        //target area: x=20..30, y=-10..-5

        let max_height = style_shot(20,30,-10,-5).unwrap();

        assert_eq!(max_height, 45);
        //end::test1[]
        
        //tag::test2[]
        let cnt = within_target(20,30,-10,-5).unwrap();
        assert_eq!(cnt, 112);
        //end::test2[]

        Ok(())
    }

}