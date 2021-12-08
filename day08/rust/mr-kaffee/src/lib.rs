use std::collections::HashMap;

// tag::parse[]
/// return a vector of tuples, each containing a vector of unique patterns and a vector of outputs
///
/// each pattern / output is represented by an integer whos bits indicate the state of segments a (bit 0) to g (bit 6)
pub fn parse(content: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut displays = Vec::new();
    for line in content.lines() {
        let parts = line
            .trim()
            .split(" | ") // separate patterns from outputs
            .map(|part| {
                part.trim()
                    .split_ascii_whitespace() // separate individual patterns
                    .map(|pattern| {
                        // convert pattern to usize
                        pattern
                            .chars()
                            .map(|char| 1usize << (char as usize - 'a' as usize))
                            .fold(0, |bits, bit| bits | bit)
                    })
                    .collect::<Vec<_>>() // patterns as usize
            })
            .collect::<Vec<_>>(); // patterns and outputs
        displays.push((parts[0].to_vec(), parts[1].to_vec()));
    }
    displays
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
    displays
        .iter()
        .map(|(patterns, outputs)| decode(patterns, outputs))
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
    let pos_patterns = HashMap::from([(2, 0b0100100), (3, 0b0100101), (4, 0b0101110)]);
    // 2 -> a _ c d e _ g
    // 3 -> a _ c d _ f g
    // 5 -> a b _ d _ f g -> unset bits match 0b0110110
    // 0 -> a b c _ e f g
    // 6 -> a b _ d e f g
    // 9 -> a b c d _ f g -> unset bits match 0b0011100
    let neg_patterns = HashMap::from([
        (2, !0b0100100),
        (3, !0b0100101),
        (4, !0b0101110),
        (5, 0b0110110),
        (6, 0b0011100),
    ]);

    // for each pattern, reduce map by impossible bits
    for pattern in patterns {
        if let Some(p) = pos_patterns.get(&pattern.count_ones()) {
            for wire in (0..7).filter(|wire| (pattern >> wire) & 1 == 1) {
                map[wire] &= p;
            }
        }
        if let Some(p) = neg_patterns.get(&pattern.count_ones()) {
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
                    _ => panic!("Invalid output {}", digit),
                }
        })
}
// end::part2[]

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
