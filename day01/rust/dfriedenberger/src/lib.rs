pub mod util;

#[cfg(test)]
mod tests {

    use crate::util::calculate_increases;
    use crate::util::calculate_window;
    
    #[test]
    fn test_it() -> Result<(), String> {

        //tag::report-testdata[]
        let vec1 = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        //end::report-testdata[] 

        //tag::test-calculate-increases[]
        assert_eq!(calculate_increases(&vec1).unwrap(), 7);
        //end::test-calculate-increases[]
        
        //tag::test-calculate-window[]
        let vec2 = calculate_window(&vec1).unwrap();
        assert_eq!(calculate_increases(&vec2).unwrap(), 5);
        //end::test-calculate-increases[]

        Ok(())
    }

}