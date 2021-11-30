pub mod parser;


#[cfg(test)]
mod tests {

    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let p = Parser {};
        assert_eq!(p.parse("abcdef"), (0,0));
        assert_eq!(p.parse("bababc"), (1,1));
        assert_eq!(p.parse("abbcde"), (1,0));
        assert_eq!(p.parse("abcccd"), (0,1));
        assert_eq!(p.parse("aabcdd"), (1,0));
        assert_eq!(p.parse("abcdee"), (1,0));
        assert_eq!(p.parse("ababab"), (0,1));

    }
}