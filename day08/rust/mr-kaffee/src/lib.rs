use std::collections::{HashMap, VecDeque};

// tag::parse[]
/// return a vector of tuples, each containing a vector of unique patterns and a vector of outputs
///
/// each pattern / output is represented by an integer whos bits indicate the state of segments a (bit 0) to g (bit 6)
pub fn parse(content: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    content
        .lines()
        .map(|line| {
            line.split(" | ") // separate patterns from outputs
                .map(|part| {
                    part.split_ascii_whitespace() // separate individual patterns
                        .map(|pattern| {
                            // convert pattern to usize
                            pattern
                                .chars()
                                .map(|char| 1 << (char as usize - 'a' as usize))
                                .fold(0, |bits, bit| bits | bit)
                        })
                        .collect::<Vec<_>>() // patterns / outputs as usize
                })
                .collect::<VecDeque<_>>() // VecDeque with one element for patterns and one for outputs
        })
        .map(|mut parts| (parts.pop_front().unwrap(), parts.pop_front().unwrap()))
        .collect()
}
// end::parse[]

// tag::part1[]
/// count unique ouput values (1 - 2 bits set, 7 - 3 bits set, 4 - 4 bits set, 8 - 7 bits set) in all outputs
pub fn solution_1(displays: &[(Vec<usize>, Vec<usize>)]) -> usize {
    displays
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|output| [2, 3, 4, 7].contains(&output.count_ones()))
                .count()
        })
        .sum()
}
// end::part1[]

// tag::part2[]
/// sum over decoded outputs
pub fn solution_2(displays: &[(Vec<usize>, Vec<usize>)]) -> usize {
    let f = if cfg!(feature = "rjplog_solution") {
        // alternative decoder if feature "rjplog_solution" is chosen
        self::decode_alt
    } else {
        self::decode
    };

    displays
        .iter()
        .map(|(patterns, outputs)| f(patterns, outputs))
        .sum()
}

/// determine wiring by unique batterns and decode output
///
/// # Example
/// ```
/// # use mr_kaffee_2021_08::*;
/// let content =
///     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
/// let displays = parse(content);
/// let (patterns, outputs) = &displays[0];
/// assert_eq!(5353, decode(patterns, outputs));
/// ```
pub fn decode(patterns: &[usize], outputs: &[usize]) -> usize {
    // map is a vector of 7 integers, one for each segment
    // each row represents a wire in a display (a = 0 to g = 6).
    // Each bit in the entry represents a segment, the wire actually controls (a = 0 to g = 6)
    let mut map: Vec<usize> = vec![(1 << 7) - 1; 7];

    // 1 -> _ _ c _ _ f _ -> set bits match 0b0100100
    // 7 -> a _ c _ _ f _ -> set bits match 0b0100101
    // 4 -> _ b c d _ f _ -> set bits match 0b0101110
    let pos_masks = HashMap::from([(2, 0b0100100), (3, 0b0100101), (4, 0b0101110)]);
    // 2 -> a _ c d e _ g
    // 3 -> a _ c d _ f g
    // 5 -> a b _ d _ f g -> unset bits match 0b0110110
    // 0 -> a b c _ e f g
    // 6 -> a b _ d e f g
    // 9 -> a b c d _ f g -> unset bits match 0b0011100
    let neg_masks = HashMap::from([
        (2, !0b0100100),
        (3, !0b0100101),
        (4, !0b0101110),
        (5, 0b0110110),
        (6, 0b0011100),
    ]);

    // for each pattern, reduce map by impossible bits
    for pattern in patterns {
        if let Some(p) = pos_masks.get(&pattern.count_ones()) {
            for wire in (0..7).filter(|wire| (pattern >> wire) & 1 == 1) {
                map[wire] &= p;
            }
        }
        if let Some(p) = neg_masks.get(&pattern.count_ones()) {
            for wire in (0..7).filter(|wire| (pattern >> wire) & 1 == 0) {
                map[wire] &= p;
            }
        }
    }

    // reduce map
    // if a row contains only one bit set, this bit is removed from every other row
    for wire_1 in 0..7 {
        if map[wire_1].count_ones() == 1 {
            for wire_2 in (0..7).filter(|wire_2| *wire_2 != wire_1) {
                map[wire_2] &= !map[wire_1];
            }
        }
    }

    // determine digits and fold to output
    outputs
        .iter()
        .map(|output| {
            // map wires to segments
            (0..7)
                .filter(|wire| (output >> wire) & 1 == 1)
                .map(|wire| map[wire])
                .fold(0, |digit, bit| digit | bit)
        })
        .fold(0, |result, digit| {
            // calculate output from digits
            10 * result
                + match digit {
                    // map digit patterns to decimal digit
                    0b1110111 => 0,
                    0b0100100 => 1,
                    0b1011101 => 2,
                    0b1101101 => 3,
                    0b0101110 => 4,
                    0b1101011 => 5,
                    0b1111011 => 6,
                    0b0100101 => 7,
                    0b1111111 => 8,
                    0b1101111 => 9,
                    _ => panic!("Invalid digit: {}", digit),
                }
        })
}
// end::part2[]

// tag::decode_alt[]
/// decode output (solution idea by https://github.com/RJPlog[RJPlog])
///
/// # Example
/// ```
/// # use mr_kaffee_2021_08::*;
/// let content =
///     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
/// let displays = parse(content);
/// let (patterns, outputs) = &displays[0];
/// assert_eq!(5353, decode(patterns, outputs));
/// ```
pub fn decode_alt(patterns: &[usize], outputs: &[usize]) -> usize {
    let mut map: Vec<usize> = vec![0; 10];
    for pattern in patterns {
        // unique segment counts
        // 1 -> _ _ c _ _ f _  2 segments
        // 7 -> a _ c _ _ f _  3 segments
        // 4 -> _ b c d _ f _  4 segments
        // 8 -> a b c d e f g  7 segments
        match pattern.count_ones() {
            2 => map[1] = *pattern,
            3 => map[7] = *pattern,
            4 => map[4] = *pattern,
            7 => map[8] = *pattern,
            _ => {} // nothing here
        }
    }
    for pattern in patterns.iter().filter(|pattern| pattern.count_ones() == 5) {
        // 2 -> a _ c d e _ g
        // 3 -> a _ c d _ f g
        // 5 -> a b _ d _ f g
        if pattern & map[1] == map[1] {
            // c | f is only contained in segments for 3
            map[3] = *pattern;
        } else if pattern & (map[4] & !map[1]) == (map[4] & !map[1]) {
            // b | c | d | f & !(c | f) = b | d is only contained in segments for 5
            map[5] = *pattern;
        } else {
            // if it is neither 3 nor 5, it must be 2
            map[2] = *pattern;
        }
    }
    for pattern in patterns.iter().filter(|pattern| pattern.count_ones() == 6) {
        // 0 -> a b c _ e f g
        // 6 -> a b _ d e f g
        // 9 -> a b c d _ f g
        if pattern & map[3] == map[3] {
            // a | c | d | f | g is only contained in the segments for 9
            map[9] = *pattern;
        } else if pattern & map[5] == map[5] {
            // a | b | d | f | g is contained in the segments for 9 (already handled) and 6
            map[6] = *pattern;
        } else {
            // if it is neither 9 nor 6, it must be 0
            map[0] = *pattern;
        }
    }

    outputs
        .iter()
        .map(|output| map.iter().position(|pattern| output == pattern).unwrap())
        .fold(0, |result, digit| 10 * result + digit)
}
// end::decode_alt[]

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
        let displays = parse(CONTENT);
        assert_eq!(26, solution_1(&displays));
    }

    #[test]
    fn test_solution_2() {
        let data = parse(CONTENT);
        let sol = solution_2(&data);
        assert_eq!(61229, sol);
    }
}
// end::tests[]
