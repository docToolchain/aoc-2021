pub mod util;

#[cfg(test)]
mod tests {
    
    use crate::util::count_bits;
    use crate::util::power_consumption;
    use crate::util::filter;
    use crate::util::count_bit;
    use crate::util::filter_by_criteria;
    use crate::util::convert2number;
    use crate::util::FilterStrategy::MostCommonBit;
    
    #[test]
    fn test_it() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")
        ];
        //end::testdata[] 

        //tag::test1[]
        let bits = count_bits(&vec1).unwrap();
        let (gamma , epsilon) = power_consumption(&bits,vec1.len()).unwrap();
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
        assert_eq!(gamma * epsilon, 198);
        //end::test1[]
        

        assert_eq!('1',count_bit(&vec1,0,MostCommonBit).unwrap());
        assert_eq!('0',count_bit(&vec1,1,MostCommonBit).unwrap());


        let v1 = filter_by_criteria(&vec1,'1',0).unwrap();
        assert_eq!(7,v1.len());
        let v2 = filter_by_criteria(&vec1,'0',1).unwrap();
        assert_eq!(7,v2.len());

        assert_eq!(4,convert2number(vec1.get(0).unwrap()).unwrap());


         //tag::test2[]
         let (oxygen_generator_rating , co2_scrubber_rating) = filter(&vec1).unwrap();
         assert_eq!(oxygen_generator_rating, 23);
         assert_eq!(co2_scrubber_rating, 10);
         assert_eq!(oxygen_generator_rating * co2_scrubber_rating, 230);
         //end::test2[]

        Ok(())
    }

}