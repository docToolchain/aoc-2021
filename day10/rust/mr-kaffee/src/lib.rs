// tag::parse[]
pub fn parse<'a>(content: &'a str) -> Vec<&'a str> {
    content.lines().collect()
}
// end::parse[]

// tag::part1[]
/// return illegal char's score if any in as ``Some`` value, otherwise ``None``
pub fn get_illegal_score(line: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' if c != stack.pop().unwrap() => return Some(3),
            ']' if c != stack.pop().unwrap() => return Some(57),
            '}' if c != stack.pop().unwrap() => return Some(1_197),
            '>' if c != stack.pop().unwrap() => return Some(25_137),
            _ => {} // nothing here
        }
    }
    None
}

/// Calculate sum of scores of illegal chars
pub fn solution_1(lines: &[&str]) -> u64 {
    lines
        .iter()
        .filter_map(|line| get_illegal_score(line))
        .sum()
}
// end::part1[]

// tag::part2[]
/// get the repair score if the line is incomplete as a ``Some`` value, otherwise return ``None``
pub fn get_repair_score(line: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | '>' | '}' | ']' if c != stack.pop().unwrap() => return None,
            _ => {} // nothing here
        }
    }

    Some(stack.iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    }))
}

/// get the middle repair score
pub fn solution_2(lines: &[&str]) -> u64 {
    let mut scores = lines
        .iter()
        .filter_map(|line| get_repair_score(line))
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_find_illegal_char() {
        let lines = parse(CONTENT);
        assert_eq!(
            vec![
                None,
                None,
                Some(1197),
                None,
                Some(3),
                Some(57),
                None,
                Some(3),
                Some(25137),
                None
            ],
            lines
                .iter()
                .map(|line| get_illegal_score(line))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_solution_1() {
        let lines = parse(CONTENT);
        assert_eq!(26_397, solution_1(&lines));
    }

    #[test]
    fn test_get_repair_score() {
        let lines = parse(CONTENT);
        assert_eq!(
            vec![
                Some(288957),
                Some(5566),
                None,
                Some(1480781),
                None,
                None,
                Some(995444),
                None,
                None,
                Some(294)
            ],
            lines
                .iter()
                .map(|line| get_repair_score(line))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_solution_2() {
        let lines = parse(CONTENT);
        assert_eq!(288_957, solution_2(&lines));
    }
}
// end::tests[]
