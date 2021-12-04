///
/// # Example
/// ```
/// assert_eq!(0, mr_kaffee_2021_05::do_something(""))
/// ```
pub fn do_something(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "";

    #[test]
    fn test_do_something() {
        assert_eq!(0, do_something(CONTENT));
    }
}
