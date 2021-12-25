pub mod reader;
pub mod parser;

#[cfg(test)]
mod tests {
    
    use crate::parser::parse_hex;
    use crate::parser::add_versions;
    use crate::parser::calc_value;

  
    #[test]
    fn test_D2FE28() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "D2FE28",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let bits = parse_hex(&vec2).unwrap();

        assert_eq!(bits.len(), 24);

        let sum = add_versions(&bits).unwrap();
        assert_eq!(sum, 6);

      

        //end::test1[]

        Ok(())
    }

    #[test]
    fn test_38006F45291200() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "38006F45291200",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let sum = add_versions(&bits).unwrap();
        assert_eq!(sum, 9);

        Ok(())
    }

    #[test]
    fn test_EE00D40C823060() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "EE00D40C823060",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let sum = add_versions(&bits).unwrap();
        assert_eq!(sum, 14);

        Ok(())
    }

    #[test]
    fn test_A0016C880162017C3686B18A3D4780() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "A0016C880162017C3686B18A3D4780",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let sum = add_versions(&bits).unwrap();
        assert_eq!(sum, 31);

        Ok(())
    }

    #[test]
    fn test_620080001611562C8802118E34() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "620080001611562C8802118E34",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let sum = add_versions(&bits).unwrap();
        assert_eq!(sum, 12);

        Ok(())
    }

    #[test]
    fn test_C200B40A82() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "C200B40A82",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 3);

        Ok(())
    }


    #[test]
    fn test_04005AC33890() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "04005AC33890",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 54);

        Ok(())
    }

    #[test]
    fn test_880086C3E88112() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "880086C3E88112",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 7);

        Ok(())
    }

    #[test]
    fn test_CE00C43D881120() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "CE00C43D881120",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 9);

        Ok(())
    }

    #[test]
    fn test_D8005AC2A8F0() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "D8005AC2A8F0",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 1);

        Ok(())
    }

    #[test]
    fn test_F600BC2D8F() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "F600BC2D8F",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 0);

        Ok(())
    }

    #[test]
    fn test_9C005AC2F8F0() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "9C005AC2F8F0",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 0);

        Ok(())
    }


    #[test]
    fn test_9C0141080250320F1802104A08() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "9C0141080250320F1802104A08",
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        let bits = parse_hex(&vec2).unwrap();

        let val = calc_value(&bits).unwrap();
        assert_eq!(val, 1);

        Ok(())
    }

 

}