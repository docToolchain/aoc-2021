// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_12::*;
/// assert_eq!(vec![0, 1], parse("0\n1"));
/// ```
pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().expect("Could not parse")).collect()
}
// end::parse[]

// tag::part1[]
pub fn solution_1(data: &[usize]) -> usize {
    data[0]
}
// end::part1[]

// tag::part2[]
pub fn solution_2(data: &[usize]) -> usize {
    data[1]
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0\n1";
    const TEST_DATA: &[usize] = &[0, 1];

    #[test]
    fn test_parse() {
        assert_eq!(TEST_DATA, parse(TEST_INPUT));
    }
}
// end::tests[]