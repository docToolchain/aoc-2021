//! # AoC 2021 / Day 01
//!
//! Solution to <https://adventofcode.com/2021/day/1>

//tag::parse[]
/// parse each line into a number
pub fn parse(content: &str) -> Vec<usize> {
    content
        .lines()
        .into_iter()
        .map(|line| line.parse().expect("Could not parse line"))
        .collect()
}
//end::parse[]

//tag::count_increase[]
/// count how often consecutive numbers increase
///
/// # Examples
/// ```
/// let data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// assert_eq!(7, mr_kaffee_2021_01::count_increase(&data));
/// ```
pub fn count_increase(data: &[usize]) -> usize {
    data.iter()
        .zip(data[1..].iter())
        .filter(|(a, b)| b > a)
        .count()
}
//end::count_increase[]

//tag::sliding_sums[]
/// calculate sliding sums over three elements
///
/// # Examples
///
/// ```
/// let data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// let sums: Vec<usize> = vec![607, 618, 618, 617, 647, 716, 769, 792];
/// assert_eq!(sums, mr_kaffee_2021_01::sliding_sums(&data));
/// ```
pub fn sliding_sums(data: &[usize]) -> Vec<usize> {
    data.iter()
        .zip(data[1..].iter())
        .zip(data[2..].iter())
        .map(|((a, b), c)| a + b + c)
        .collect()
}
//end::sliding_sums[]

//tag::generic_solution[]
/// count how often numbers with distance `offset` increase
///
/// # Examples
/// ```
/// let data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
/// assert_eq!(7, mr_kaffee_2021_01::count_increase_with_offset(&data, 1));
/// assert_eq!(5, mr_kaffee_2021_01::count_increase_with_offset(&data, 3));
/// ```
pub fn count_increase_with_offset(data: &[usize], offset: usize) -> usize {
    data.iter()
        .zip(data[offset..].iter())
        .filter(|(a, b)| b > a)
        .count()
}
//end::generic_solution[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_parse() {
        let data: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(parse(CONTENT), data, "Parse failed");
    }
}
//end::tests[]
