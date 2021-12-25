use std::collections::HashMap;

// tag::parse[]
/// porse content into a vector of chars and a map with pairs of chars as keys and the
/// char to be inserted as value
pub fn parse(content: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut parts = content.split("\n\n");

    let template = parts
        .next()
        .expect("No template")
        .chars()
        .collect::<Vec<_>>();

    let rules = parts
        .next()
        .expect("No rules")
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().expect("No first char");
            let b = chars.next().expect("No second char");
            let c = chars.skip(4).next().expect("No resulting char");
            ((a, b), c)
        })
        .collect::<HashMap<_, _>>();

    (template, rules)
}
// end::parse[]

// tag::simulate_rounds[]
/// simulate given number of rounds starting from polymer template and using given 
/// rules
pub fn simulate_rounds(
    template: &[char],
    rules: &HashMap<(char, char), char>,
    rounds: usize,
) -> usize {
    // map of pairs to number of occurances of those
    let mut pairs = HashMap::new();
    for (c1, c2) in template.iter().zip(template.iter().skip(1)) {
        let cnt = pairs.entry((*c1, *c2)).or_insert(0usize);
        *cnt += 1;
    }

    // in each round, update pairs
    for _ in 0..rounds {
        let mut upd = HashMap::new();
        for ((c1, c2), cnt0) in &pairs {
            if let Some(d) = rules.get(&(*c1, *c2)) {
                // if pair is found in rules, replace (c1, c2) by (c1, d) and (d, c2)
                let cnt = upd.entry((*c1, *d)).or_insert(0);
                *cnt += cnt0;
                let cnt = upd.entry((*d, *c2)).or_insert(0);
                *cnt += cnt0;
            } else {
                // if pair is not found in rules, keep pair
                let cnt = upd.entry((*c1, *c2)).or_insert(0);
                *cnt += cnt0;
            }
        }
        pairs = upd;
    }

    // count symbols in pairs
    // each symbol contributes to two pairs except for the first and the last symbol
    // start with count = 1 for first and last symbol to consistently count every symbol 
    // twice
    let mut counts = HashMap::new();
    counts.insert(template[0], 1);
    counts.insert(template[template.len() - 1], 1);
    for ((c1, c2), cnt) in &pairs {
        let count = counts.entry(*c1).or_insert(0);
        *count += cnt;
        let count = counts.entry(*c2).or_insert(0);
        *count += cnt;
    }

    // get (twice the) count for most and less frequent symbol
    let min = counts.values().min().expect("No min");
    let max = counts.values().max().expect("No max");

    // return difference, divide by 2 because every symbol is counted twice
    (max - min) / 2
}
// end::simulate_rounds[]

pub fn solution_1(template: &[char], rules: &HashMap<(char, char), char>) -> usize {
    simulate_rounds(template, rules, 10)
}

pub fn solution_2(template: &[char], rules: &HashMap<(char, char), char>) -> usize {
    simulate_rounds(template, rules, 40)
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    const TEST_TEMPLATE: [char; 4] = ['N', 'N', 'C', 'B'];
    const TEST_RULES: [((char, char), char); 16] = [
        (('C', 'H'), 'B'),
        (('H', 'H'), 'N'),
        (('C', 'B'), 'H'),
        (('N', 'H'), 'C'),
        (('H', 'B'), 'C'),
        (('H', 'C'), 'B'),
        (('H', 'N'), 'C'),
        (('N', 'N'), 'C'),
        (('B', 'H'), 'H'),
        (('N', 'C'), 'B'),
        (('N', 'B'), 'B'),
        (('B', 'N'), 'B'),
        (('B', 'B'), 'N'),
        (('B', 'C'), 'B'),
        (('C', 'C'), 'N'),
        (('C', 'N'), 'C'),
    ];

    #[test]
    fn test_parse() {
        let (template, rules) = parse(&CONTENT);
        assert_eq!(Vec::from(TEST_TEMPLATE), template);
        assert_eq!(HashMap::from(TEST_RULES), rules);
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(
            1_588,
            solution_1(&Vec::from(TEST_TEMPLATE), &HashMap::from(TEST_RULES))
        );
    }
}
// end::tests[]
