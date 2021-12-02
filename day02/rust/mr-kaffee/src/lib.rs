//tag::calc_position[]
/// Calculate positions
///
/// # Example
/// ```
/// assert_eq!((5, 0), mr_kaffee_2021_02::calc_position("forward 5"));
/// assert_eq!((0, 5), mr_kaffee_2021_02::calc_position("down 5"));
/// assert_eq!((0, -5), mr_kaffee_2021_02::calc_position("up 5"));
/// ```
pub fn calc_position(input: &str) -> (isize, isize) {
    input
        .lines()
        .fold((0, 0), |(x, y), line| match line.chars().next() {
            Some('u') => (x, y - line[3..].parse::<isize>().expect("Invalid value")),
            Some('d') => (x, y + line[5..].parse::<isize>().expect("Invalid value")),
            Some('f') => (x + line[8..].parse::<isize>().expect("Invalid value"), y),
            _ => panic!("Invalid direction!"),
        })
}
//end::calc_position[]

//tag::calc_position_with_aim[]
/// Calculate positions with aim
///
/// # Example
/// ```
/// assert_eq!((5, 0, 0), mr_kaffee_2021_02::calc_position_with_aim("forward 5"));
/// assert_eq!((5, 0, 5), mr_kaffee_2021_02::calc_position_with_aim("forward 5\ndown 5"));
/// assert_eq!(
///     (13, 40, 5),
///     mr_kaffee_2021_02::calc_position_with_aim("forward 5\ndown 5\nforward 8")
/// );
/// ```
pub fn calc_position_with_aim(input: &str) -> (isize, isize, isize) {
    input
        .lines()
        .fold((0, 0, 0), |(x, y, aim), line| match line.chars().next() {
            Some('u') => (
                x,
                y,
                aim - line[3..].parse::<isize>().expect("Invalid value"),
            ),
            Some('d') => (
                x,
                y,
                aim + line[5..].parse::<isize>().expect("Invalid value"),
            ),
            Some('f') => {
                let v = line[8..].parse::<isize>().expect("Invalid value");
                (x + v, y + aim * v, aim)
            }
            _ => panic!("Invalid direction!"),
        })
}
//end::calc_position_with_aim[]

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_calc_position() {
        let pos = calc_position(CONTENT);
        assert_eq!((15, 10), pos);
        assert_eq!(150, pos.0 * pos.1);
    }

    #[test]
    fn test_calc_position_with_aim() {
        assert_eq!((15, 60, 10), calc_position_with_aim(CONTENT));
    }
}
