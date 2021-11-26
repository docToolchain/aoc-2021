//tag::function[]
/// Write string to console
///
/// # Examples
///
/// ```
/// // running documentation examples as tests is a great idea in general; in this specific case
/// // there might be little value in the code and test
/// let s = "Hello World!";
/// mr_kaffee_2021_00::write_output(&s);
/// ```
pub fn write_output(s: &str) {
    println!("{}", s);
}
//end::function[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_output() {
        // very stupid test
        write_output("Hello World!");
    }
}
//end::tests[]