// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_08::*;
/// assert_eq!(vec![0, 1], parse("0,1"));
/// ```
pub fn parse(content: &str) -> Vec<usize> {
    content
        .trim()
        .split(',')
        .map(|v| v.parse().expect("Could not parse number"))
        .collect()
}
// end::parse[]

pub fn solution_1(data: &[usize]) -> usize {
    data.len()
}

pub fn solution_2(data: &[usize]) -> isize {
    data.iter()
        .map(|v| *v as isize)
        .fold(0, |acc, v| acc + v * (1 - 2 * (v & 1)))
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "0,1";
    const DATA: &'static [usize] = &[0, 1];

    #[test]
    fn test_parse() {
        assert_eq!(DATA, parse(CONTENT));
    }
}
// end::tests[]
