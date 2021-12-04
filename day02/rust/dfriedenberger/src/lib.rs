pub mod util;

#[cfg(test)]
mod tests {
    
    use crate::util::parse;
    use crate::util::course;
    use crate::util::course2;

    #[test]
    fn test_it() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2")
        ];
        //end::testdata[] 

        //tag::test1[]
        let parsed = parse(&vec1).unwrap();
        let (position,deep) = course(&parsed).unwrap();
        assert_eq!(position, 15);
        assert_eq!(deep, 10);
        assert_eq!(position * deep, 150);
        //end::test1[]
        
         //tag::test2[]
         let (position,deep) = course2(&parsed).unwrap();
         assert_eq!(position, 15);
         assert_eq!(deep, 60);
         assert_eq!(position * deep, 900);
         //end::test2[]

        Ok(())
    }

}