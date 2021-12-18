// tag::token[]
/// holds tokens of a snail number
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    Op,
    Cl,
    Val(usize),
}
// end::token[]

// tag::parse[]
/// # Examples
/// ```
/// # use mr_kaffee_2021_18::*;
/// assert_eq!(
///   vec![
///     vec![
///       Token::Op,
///       Token::Op,
///       Token::Val(0),
///       Token::Val(1),
///       Token::Cl,
///       Token::Val(1),
///       Token::Cl
///     ]
///   ],
///   parse("[[0,1],1]"));
/// ```
pub fn parse(content: &str) -> Vec<Vec<Token>> {
    content
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| *c != ',')
                .map(|c| match c {
                    '[' => Token::Op,
                    ']' => Token::Cl,
                    '0'..='9' => Token::Val(c as usize - '0' as usize),
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
// end::parse[]

// tag::explode[]
/// perform single explode
///
/// returns true if any explode performed, otherwise false
pub fn explode(snail: &mut Vec<Token>) -> bool {
    let mut level = 0;
    let mut last_val = 0;
    let mut number_seen = false;
    for k in 0..snail.len() {
        match snail[k] {
            Token::Op => {
                number_seen = false;
                level += 1;
            }
            Token::Cl => {
                number_seen = false;
                level -= 1;
            }
            Token::Val(val) => {
                if number_seen && level == 5 {
                    let prev = (0..k - 2)
                        .rev()
                        .filter_map(|j| {
                            if let Token::Val(v) = snail[j] {
                                Some((j, v))
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some((j, x)) = prev {
                        snail[j] = Token::Val(x + last_val);
                    }

                    let next = (k + 2..snail.len())
                        .filter_map(|j| {
                            if let Token::Val(v) = snail[j] {
                                Some((j, v))
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some((j, x)) = next {
                        snail[j] = Token::Val(x + val);
                    }

                    // remove elements k-1, k, k+1
                    snail.drain(k - 1..k + 2);
                    snail[k - 2] = Token::Val(0);

                    return true;
                } else {
                    last_val = val;
                    number_seen = true;
                }
            }
        }
    }

    false
}
// end::explode[]

// tag::split[]
/// perform single split
///
/// returns true if any split performed, otherwise false
pub fn split(snail: &mut Vec<Token>) -> bool {
    let find = snail
        .iter()
        .enumerate()
        .filter_map(|(k, e)| {
            if let Token::Val(v) = e {
                if v > &9 {
                    Some((k, v))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .next();

    if let Some((k, v)) = find {
        let a = v >> 1;
        let b = v - a;
        snail.splice(
            k..k + 1,
            [Token::Op, Token::Val(a), Token::Val(b), Token::Cl],
        );

        true
    } else {
        false
    }
}
// end::split[]

// tag::reduce[]
/// reduce snail number
pub fn reduce(snail: &mut Vec<Token>) {
    while explode(snail) || split(snail) {}
}
// end::reduce[]

// tag::sum[]
/// calculate sum over snailnumbers at given indices
pub fn sum(snail: &[Vec<Token>], mut indices: impl Iterator<Item = usize>) -> Vec<Token> {
    let mut sum = snail[indices.next().expect("Empty indices")].to_owned();
    for k in indices {
        sum.insert(0, Token::Op);
        for e in &snail[k] {
            sum.push(*e);
        }
        sum.push(Token::Cl);
        reduce(&mut sum);
    }
    sum
}
// end::sum[]

// tag::magnitude[]
/// recursively calculate magnitude for snailnumber at given position
///
/// returns magnitude and position
pub fn magnitude(snail: &[Token], k: usize) -> (usize, usize) {
    assert_eq!(Token::Op, snail[k], "Do not start with open");

    let (a, j) = match snail[k + 1] {
        Token::Op => magnitude(&snail, k + 1),
        Token::Val(v) => (v, k + 2),
        Token::Cl => panic!("Unexpected close tag at {}", k + 1),
    };
    let (b, j) = match snail[j] {
        Token::Op => magnitude(&snail, j),
        Token::Val(v) => (v, j + 1),
        Token::Cl => panic!("Unexpected close tag at {}", j),
    };

    assert_eq!(Token::Cl, snail[j], "Do not end with close");

    (3 * a + 2 * b, j + 1)
}
// end::magnitude[]

// tag::part1[]
pub fn solution_1(snails: &[Vec<Token>]) -> usize {
    let sum = sum(snails, 0..snails.len());
    let (m, _) = magnitude(&sum, 0);
    m
}
// end::part1[]

// tag::part2[]
pub fn solution_2(snails: &[Vec<Token>]) -> usize {
    let mut max = 0;
    for k1 in 0..snails.len() {
        for k2 in 0..snails.len() {
            if k1 == k2 {
                continue;
            }

            let sum = sum(snails, [k1, k2].into_iter());
            let (m, _) = magnitude(&sum, 0);
            if m > max {
                max = m;
            }
        }
    }

    max
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let mut snails = parse("[[[[[9,8],1],2],3],4]\n[[[[0,9],2],3],4]");
        assert!(explode(&mut snails[0]), "Did not explode");
        assert_eq!(snails[0], snails[1]);

        let mut snails =
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]\n[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        assert!(explode(&mut snails[0]));
        assert_eq!(snails[0], snails[1]);
    }

    #[test]
    fn test_split() {
        let mut snails = vec![
            vec![
                Token::Op,
                Token::Op,
                Token::Val(1),
                Token::Val(15),
                Token::Cl,
                Token::Val(11),
                Token::Cl,
            ],
            vec![
                Token::Op,
                Token::Op,
                Token::Val(1),
                Token::Op,
                Token::Val(7),
                Token::Val(8),
                Token::Cl,
                Token::Cl,
                Token::Val(11),
                Token::Cl,
            ],
        ];
        assert!(split(&mut snails[0]), "Did not split");
        assert_eq!(snails[0], snails[1]);
    }

    #[test]
    fn test_reduce() {
        let mut snails =
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]\n[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        reduce(&mut snails[0]);
        assert_eq!(snails[0], snails[1]);
    }

    #[test]
    fn test_sum() {
        let snails = parse(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        );

        let sum = sum(&snails, 0..snails.len());
        assert_eq!(
            parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")[0],
            sum
        );
    }

    #[test]
    fn test_magnitude() {
        let snails = parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        let (m, j) = magnitude(&snails[0], 0);
        assert_eq!(4140, m);
        assert_eq!(snails[0].len(), j);
    }
}
// end::tests[]

pub mod recursive {
    use std::{fmt, iter::Peekable};

    // tag::snail[]
    #[derive(Clone, PartialEq, Eq)]
    pub enum Snail {
        Val(usize),
        Pair(Box<Snail>, Box<Snail>),
    }

    impl fmt::Display for Snail {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Val(v) => write!(f, "{}", v),
                Self::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
            }
        }
    }

    impl fmt::Debug for Snail {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Val(v) => write!(f, "{}", v),
                Self::Pair(lhs, rhs) => write!(f, "[{},{}]", lhs, rhs),
            }
        }
    }

    impl Snail {
        /// parse a string representation of a snail number
        ///
        /// # Examples
        /// ```
        /// # use mr_kaffee_2021_18::*;
        /// let snail = Snail::parse("[[1,2]\n, 3]");
        /// assert_eq!("[[1,2],3]", format!("{}", snail));
        /// ```
        pub fn parse(data: &str) -> Self {
            Self::parse_recursive(&mut data.chars().filter(|c| !c.is_whitespace()).peekable())
        }

        fn parse_recursive(data: &mut Peekable<impl Iterator<Item = char>>) -> Self {
            let token = data.next();
            match token {
                Some('[') => {
                    let lhs = Self::parse_recursive(data);
                    assert_eq!(Some(','), data.next());
                    let rhs = Self::parse_recursive(data);
                    assert_eq!(Some(']'), data.next());
                    Snail::Pair(Box::new(lhs), Box::new(rhs))
                }
                Some(c) if c >= '0' && c <= '9' => {
                    let mut v = c as usize - '0' as usize;
                    while let Some(d) = data
                        .peek()
                        .filter(|c| c.is_ascii_digit())
                        .map(|c| *c as usize - '0' as usize)
                    {
                        data.next();
                        v = 10 * v + d;
                    }
                    Snail::Val(v)
                }
                _ => panic!("Unexpected token: {:?}", token),
            }
        }

        /// get the value wrapped in an option if this is a [Snail::Val]. Otherwise return ``None``.
        pub fn get_value(&self) -> Option<usize> {
            match self {
                Self::Val(a) => Some(*a),
                _ => None,
            }
        }

        fn increment_first(&mut self, inc: usize) {
            match self {
                Self::Pair(lhs, _) => {
                    if let Some(val) = lhs.get_value() {
                        let _ = std::mem::replace(lhs, Box::new(Snail::Val(val + inc)));
                    } else {
                        lhs.increment_first(inc);
                    }
                }
                _ => {}
            }
        }

        fn increment_last(&mut self, inc: usize) {
            match self {
                Self::Pair(_, rhs) => {
                    if let Some(val) = rhs.get_value() {
                        let _ = std::mem::replace(rhs, Box::new(Snail::Val(val + inc)));
                    } else {
                        rhs.increment_last(inc);
                    }
                }
                _ => {}
            }
        }

        fn explode(&mut self, level: usize) -> (Option<Self>, Option<(usize, usize)>) {
            match self {
                Self::Val(_) => (None, None),
                Self::Pair(lhs, rhs) => {
                    if level == 4 {
                        if let (Some(a), Some(b)) = (lhs.get_value(), rhs.get_value()) {
                            (Some(Self::Val(0)), Some((a, b)))
                        } else {
                            (None, None)
                        }
                    } else if level < 4 {
                        if let (replace, Some((a, b))) = lhs.explode(level + 1) {
                            if let Some(val) = rhs.get_value() {
                                // increment rhs with b
                                let _ = std::mem::replace(rhs, Box::new(Self::Val(val + b)));
                            } else {
                                // increment first number in rhs with b
                                rhs.increment_first(b);
                            }

                            if let Some(replace) = replace {
                                // replace exploded value
                                let _ = std::mem::replace(lhs, Box::new(replace));
                            }
                            (None, Some((a, 0)))
                        } else if let (replace, Some((a, b))) = rhs.explode(level + 1) {
                            if let Some(val) = lhs.get_value() {
                                // increment lhs with a
                                let _ = std::mem::replace(lhs, Box::new(Self::Val(val + a)));
                            } else {
                                // increment last number in lhs with a
                                lhs.increment_last(a);
                            }

                            if let Some(replace) = replace {
                                // replace exploded value
                                let _ = std::mem::replace(rhs, Box::new(replace));
                            }

                            (None, Some((0, b)))
                        } else {
                            (None, None)
                        }
                    } else {
                        (None, None)
                    }
                }
            }
        }

        fn split(&mut self) -> bool {
            if let Self::Pair(lhs, rhs) = self {
                if let Some(val) = lhs.get_value() {
                    if val > 9 {
                        let a = Box::new(Self::Val(val >> 1));
                        let b = Box::new(Self::Val((val >> 1) + (val & 1)));
                        let _ = std::mem::replace(lhs, Box::new(Self::Pair(a, b)));
                        return true;
                    }
                }
                if lhs.split() {
                    return true;
                }
                if let Some(val) = rhs.get_value() {
                    if val > 9 {
                        let a = Box::new(Self::Val(val >> 1));
                        let b = Box::new(Self::Val((val >> 1) + (val & 1)));
                        let _ = std::mem::replace(rhs, Box::new(Self::Pair(a, b)));
                        return true;
                    }
                }
                return rhs.split();
            }

            false
        }

        pub fn reduce(&mut self) {
            loop {
                let (_, r) = self.explode(0);
                if r.is_none() && !self.split() {
                    break;
                }
            }
        }

        pub fn add(&self, other: &Snail) -> Self {
            let mut sum = Self::Pair(Box::new(self.clone()), Box::new(other.clone()));
            sum.reduce();
            sum
        }

        pub fn magnitude(&self) -> usize {
            match self {
                Self::Val(v) => *v,
                Self::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            }
        }
    }
    // end::snail[]

    pub fn parse(content: &str) -> Vec<Snail> {
        content.lines().map(|line| Snail::parse(line)).collect()
    }

    pub fn solution_1(snails: &[Snail]) -> usize {
        let mut sum = snails[0].clone();
        for snail in snails.iter().skip(1) {
            sum = sum.add(snail);
        }
        sum.magnitude()
    }

    pub fn solution_2(snails: &[Snail]) -> usize {
        let mut max = 0;
        for k1 in 0..snails.len() {
            for k2 in 0..snails.len() {
                if k1 == k2 {
                    continue;
                }

                let m = snails[k1].add(&snails[k2]).magnitude();
                if m > max {
                    max = m;
                }
            }
        }

        max
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_snail_magnitude() {
            let snail =
                Snail::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
            assert_eq!(4140, snail.magnitude());
        }

        #[test]
        fn test_snail_parse() {
            let data = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
                [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
                [[[[0,7],4],[15,[0,13]]],[1,1]]
                [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
                [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
                [[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

            for line in data.lines() {
                assert_eq!(line, format!("{}", Snail::parse(line)));
            }
        }

        #[test]
        fn test_snail_explode() {
            let mut snail = Snail::parse("[[[[[9,8],1],2],3],4]");
            let (_, exploded) = snail.explode(0);
            assert!(exploded.is_some(), "Did not explode");
            assert_eq!(Snail::parse("[[[[0,9],2],3],4]"), snail);

            let mut snail = Snail::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
            let (_, exploded) = snail.explode(0);
            assert!(exploded.is_some(), "Did not explode");
            assert_eq!(Snail::parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]"), snail);

            let mut snail = Snail::parse(
                "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            );
            let (_, exploded) = snail.explode(0);
            println!("Exploded: {:?}", exploded);
            assert_eq!(
                "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                format!("{}", snail)
            );
        }

        #[test]
        fn test_snail_split() {
            let mut snail = Snail::parse("[[[[0,7],4],[15,[0,13]]],[1,1]]");
            assert!(snail.split(), "Did not split");
            assert_eq!(Snail::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"), snail);

            let mut snail = Snail::parse("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]");
            assert!(snail.split(), "Did not split");
            assert_eq!(Snail::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"), snail);

            let mut snail = Snail::parse("[[[9,10],20],[8,[9,0]]]");
            assert!(snail.split(), "Did not split");
            assert_eq!("[[[9,[5,5]],20],[8,[9,0]]]", format!("{}", snail));

            let mut snail = Snail::parse("[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,10],20],[8,[9,0]]]]");
            assert!(snail.split(), "Did not split");
            assert_eq!(
                "[[[[7,7],[7,8]],[[9,5],[8,0]]],[[[9,[5,5]],20],[8,[9,0]]]]",
                format!("{}", snail)
            );
        }

        #[test]
        fn test_snail_reduce() {
            let mut snail = Snail::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
            snail.reduce();
            assert_eq!(Snail::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), snail);
        }

        #[test]
        fn test_snail_clone() {
            let mut snail_a = Snail::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
            let snail_b = snail_a.clone();
            snail_a.reduce();

            assert_eq!(
                Snail::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
                snail_b,
                "Clone modified"
            );
            assert_eq!(
                Snail::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
                snail_a,
                "Cloned snail not reduced"
            );
        }

        #[test]
        fn test_snail_add() {
            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
                        .add(&Snail::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"))
                ),
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
                "Sum 1 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
                        .add(&Snail::parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"))
                ),
                "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
                "Sum 2 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]")
                        .add(&Snail::parse(
                            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
                        ))
                ),
                "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
                "Sum 3 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]")
                        .add(&Snail::parse("[7,[5,[[3,8],[1,4]]]]"))
                ),
                "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
                "Sum 4 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]")
                        .add(&Snail::parse("[[2,[2,2]],[8,[8,1]]]"))
                ),
                "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
                "Sum 5 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]")
                        .add(&Snail::parse("[2,9]"))
                ),
                "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
                "Sum 6 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]")
                        .add(&Snail::parse("[1,[[[9,3],9],[[9,0],[0,7]]]]"))
                ),
                "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
                "Sum 7 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]")
                        .add(&Snail::parse("[[[5,[7,4]],7],1]"))
                ),
                "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
                "Sum 8 failed"
            );

            assert_eq!(
                format!(
                    "{}",
                    Snail::parse("[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]")
                        .add(&Snail::parse("[[[[4,2],2],6],[8,7]]"))
                ),
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                "Sum 9 failed"
            );

            let lines = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
                [[[5,[2,8]],4],[5,[[9,9],0]]]
                [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
                [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
                [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
                [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
                [[[[5,4],[7,7]],8],[[8,3],8]]
                [[9,3],[[9,9],[6,[4,9]]]]
                [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
                [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
            let sum = lines
                .lines()
                .map(|line| Snail::parse(line))
                .fold(None as Option<Snail>, |sum, snail| {
                    sum.map(|sum| sum.add(&snail)).or(Some(snail))
                });

            assert_eq!(
                Some(String::from(
                    "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
                )),
                sum.map(|sum| format!("{}", sum)),
                "Sum multi failed"
            );
        }
    }
}
