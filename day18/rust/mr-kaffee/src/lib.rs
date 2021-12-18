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
pub fn explode(snailnumber: &mut Vec<Token>) -> bool {
    let mut level = 0;
    let mut last_val = 0;
    let mut number_seen = false;
    for k in 0..snailnumber.len() {
        match snailnumber[k] {
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
                            if let Token::Val(v) = snailnumber[j] {
                                Some((j, v))
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some((j, x)) = prev {
                        snailnumber[j] = Token::Val(x + last_val);
                    }

                    let next = (k + 2..snailnumber.len())
                        .filter_map(|j| {
                            if let Token::Val(v) = snailnumber[j] {
                                Some((j, v))
                            } else {
                                None
                            }
                        })
                        .next();
                    if let Some((j, x)) = next {
                        snailnumber[j] = Token::Val(x + val);
                    }

                    assert_eq!(
                        Token::Cl,
                        snailnumber.remove(k + 1),
                        "Expected to remove close"
                    );
                    assert_eq!(
                        Token::Val(val),
                        snailnumber.remove(k),
                        "Expected to remove 2nd number"
                    );
                    assert_eq!(
                        Token::Val(last_val),
                        snailnumber.remove(k - 1),
                        "Expected to remove 1st number"
                    );
                    snailnumber[k - 2] = Token::Val(0);

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
pub fn split(snailnumber: &mut Vec<Token>) -> bool {
    let find = snailnumber
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
        let a = v / 2;
        let b = v - a;
        snailnumber[k] = Token::Op;
        snailnumber.insert(k + 1, Token::Val(a));
        snailnumber.insert(k + 2, Token::Val(b));
        snailnumber.insert(k + 3, Token::Cl);

        true
    } else {
        false
    }
}
// end::split[]

// tag::reduce[]
/// reduce snail number
pub fn reduce(snailnumber: &mut Vec<Token>) {
    while explode(snailnumber) || split(snailnumber) {}
}
// end::reduce[]

// tag::sum[]
/// calculate sum over snailnumbers at given indices
pub fn sum(snailnumbers: &[Vec<Token>], mut indices: impl Iterator<Item = usize>) -> Vec<Token> {
    let mut sum = snailnumbers[indices.next().expect("Empty indices")].to_owned();
    for k in indices {
        sum.insert(0, Token::Op);
        for e in &snailnumbers[k] {
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
pub fn magnitude(snailnumber: &[Token], k: usize) -> (usize, usize) {
    assert_eq!(Token::Op, snailnumber[k], "Do not start with open");

    let (a, j) = match snailnumber[k + 1] {
        Token::Op => magnitude(&snailnumber, k + 1),
        Token::Val(v) => (v, k + 2),
        Token::Cl => panic!("Unexpected close tag at {}", k + 1),
    };
    let (b, j) = match snailnumber[j] {
        Token::Op => magnitude(&snailnumber, j),
        Token::Val(v) => (v, j + 1),
        Token::Cl => panic!("Unexpected close tag at {}", j),
    };

    assert_eq!(Token::Cl, snailnumber[j], "Do not end with close");

    (3 * a + 2 * b, j + 1)
}
// end::magnitude[]

// tag::part1[]
pub fn solution_1(snailnumbers: &[Vec<Token>]) -> usize {
    let sum = sum(snailnumbers, 0..snailnumbers.len());
    let (m, _) = magnitude(&sum, 0);
    m
}
// end::part1[]

// tag::part2[]
pub fn solution_2(snailnumbers: &[Vec<Token>]) -> usize {
    let mut max = 0;
    for k1 in 0..snailnumbers.len() {
        for k2 in 0..snailnumbers.len() {
            if k1 == k2 {
                continue;
            }

            let sum = sum(snailnumbers, [k1, k2].into_iter());
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
        let mut snailnumbers = parse("[[[[[9,8],1],2],3],4]\n[[[[0,9],2],3],4]");
        assert!(explode(&mut snailnumbers[0]), "Did not explode");
        assert_eq!(snailnumbers[0], snailnumbers[1]);

        let mut snailnumbers =
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]\n[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
        assert!(explode(&mut snailnumbers[0]));
        assert_eq!(snailnumbers[0], snailnumbers[1]);
    }

    #[test]
    fn test_split() {
        let mut snailnumbers = vec![
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
        assert!(split(&mut snailnumbers[0]), "Did not split");
        assert_eq!(snailnumbers[0], snailnumbers[1]);
    }

    #[test]
    fn test_reduce() {
        let mut snailnumbers =
            parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]\n[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

        reduce(&mut snailnumbers[0]);
        assert_eq!(snailnumbers[0], snailnumbers[1]);
    }

    #[test]
    fn test_sum() {
        let snailnumbers = parse(
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

        let sum = sum(&snailnumbers, 0..snailnumbers.len());
        assert_eq!(
            parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")[0],
            sum
        );
    }

    #[test]
    fn test_magnitude() {
        let snailnumbers = parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        let (m, j) = magnitude(&snailnumbers[0], 0);
        assert_eq!(4140, m);
        assert_eq!(snailnumbers[0].len(), j);
    }
}
// end::tests[]
