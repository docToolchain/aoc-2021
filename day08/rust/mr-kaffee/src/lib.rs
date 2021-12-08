// tag::parse[]
pub fn parse(content: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut inputs = Vec::new();
    for line in content.lines() {
        let parts = line.trim().split(" | ");
        let mut parts_vec = Vec::new();
        for part in parts {
            let patterns = part.trim().split_ascii_whitespace();
            let mut patterns_vec = Vec::new();
            for pattern in patterns {
                let chars = pattern.chars().collect::<String>();
                patterns_vec.push(chars);
            }
            parts_vec.push(patterns_vec);
        }
        if parts_vec.len() != 2 {
            panic!("Bad number of parts in {:?}", parts_vec);
        }
        inputs.push((parts_vec[0].to_vec(), parts_vec[1].to_vec()));
    }
    inputs
}
// end::parse[]

pub fn solution_1(data: &[(Vec<String>, Vec<String>)]) -> usize {
    data.iter()
        .map(|(_, codes)| {
            codes
                .iter()
                .filter(|code| {
                    code.len() == 2 || code.len() == 3 || code.len() == 4 || code.len() == 7
                })
                .count()
        })
        .sum()
}

pub fn solution_2(data: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut result = 0;
    for (patterns, outputs) in data {
        result += decode(patterns, outputs);
    }
    result
}

pub fn decode(patterns: &[String], outputs: &[String]) -> usize {
    // 1 -> 2     _ _ c _ _ f _
    // 7 -> 3     a _ c _ _ f _
    // 4 -> 4     _ b c d _ f _

    // 2 -> 5     a _ c d e _ g
    // 3 -> 5     a _ c d _ f g
    // 5 -> 5     a b _ d _ f g

    // 0 -> 6     a b c _ e f g
    // 6 -> 6     a b _ d e f g
    // 9 -> 6     a b c d _ f g

    // 8 -> 7     a b c d e f g

    let mut map = vec![true; 7 * 7];
    let off = 'a' as usize;

    for pattern in patterns {
        match pattern.len() {
            2 => {
                // two chars can only be c or f
                for c in pattern.chars() {
                    for d in ['a', 'b', 'd', 'e', 'g'] {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
                for d in ['c', 'f'] {
                    for c in "abcdefg"
                        .chars()
                        .filter(|c| !pattern.chars().any(|d| *c == d))
                    {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
            }
            3 => {
                // three chars can only be a c or f
                for c in pattern.chars() {
                    for d in ['b', 'd', 'e', 'g'] {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
                for d in ['a', 'c', 'f'] {
                    for c in "abcdefg"
                        .chars()
                        .filter(|c| !pattern.chars().any(|d| *c == d))
                    {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
            }
            4 => {
                // four chars can only be b c d or f
                for c in pattern.chars() {
                    for d in ['a', 'e', 'g'] {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
                for d in ['b', 'c', 'd', 'f'] {
                    for c in "abcdefg"
                        .chars()
                        .filter(|c| !pattern.chars().any(|d| *c == d))
                    {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
            }
            5 => {
                // unset chars can only be b c e or f
                for c in "abcdefg"
                    .chars()
                    .filter(|c| !pattern.chars().any(|d| *c == d))
                {
                    for d in ['a', 'd', 'g'] {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
            }
            6 => {
                // unset chars can only be c d or e
                for c in "abcdefg"
                    .chars()
                    .filter(|c| !pattern.chars().any(|d| *c == d))
                {
                    for d in ['a', 'b', 'f', 'g'] {
                        map[(d as usize) - off + 7 * ((c as usize) - off)] = false;
                    }
                }
            }
            7 => {
                // can't deduce anything
            }
            _ => panic!("Invalid pattern"),
        }
    }

    for c in 0..7 {
        let indices = map[7 * c..7 * (c + 1)]
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();
        if indices.len() == 1 {
            for d in 0..7 {
                if d == c {
                    continue;
                }
                map[indices[0] + 7 * d] = false;
            }
        }
    }

    let mut result = 0;
    for output in outputs {
        let output = output
            .chars()
            .map(|c| {
                map[7 * (c as usize - off)..7 * (c as usize - off) + 7]
                    .iter()
                    .enumerate()
                    .find(|(_, d)| **d)
                    .expect("Nothing found")
            })
            .map(|(a, _)| (a + off) as u8 as char)
            .collect::<Vec<_>>();
        let output = "abcdefg"
            .chars()
            .filter(|c| output.contains(c))
            .collect::<String>();

        result = 10 * result
            + match output.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => panic!("Invalid output {}", output),
            }
    }

    result
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_solution_1() {
        let data = parse(CONTENT);
        let sol = solution_1(&data);
        assert_eq!(26, sol);
    }

    #[test]
    fn test_decode() {
        let data = parse(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let (patterns, outputs) = &data[0];
        let result = decode(patterns, outputs);
        assert_eq!(5353, result);
    }

    #[test]
    fn test_solution_2() {
        let data = parse(CONTENT);
        let sol = solution_2(&data);
        assert_eq!(61229, sol);
    }
}
// end::tests[]
