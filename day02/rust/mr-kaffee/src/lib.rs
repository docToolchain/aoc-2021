//tag::command[]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Up(isize),
    Down(isize),
    Forward(isize),
}

impl Command {
    /// parse command
    ///
    /// # Examples
    /// ```
    /// # use mr_kaffee_2021_02::*;
    /// assert_eq!(Command::Up(5), Command::parse("up 5"));
    /// assert_eq!(Command::Down(6), Command::parse("down 6"));
    /// assert_eq!(Command::Forward(-2), Command::parse("forward -2"));
    /// ```
    /// 
    /// ```should_panic
    /// # use mr_kaffee_2021_02::*;
    /// // this will panic since the first part is not a valid command
    /// let cmd = Command::parse("invalid-command 7");
    /// ```
    /// 
    /// ```should_panic
    /// # use mr_kaffee_2021_02::*;
    /// // this will panic since the two parts are not separated by a single blank
    /// let cmd = Command::parse("up\t7");
    /// ```
    /// 
    /// ```should_panic
    /// # use mr_kaffee_2021_02::*;
    /// // this will panic since the second part is not a number
    /// let cmd = Command::parse("down not-a-number");
    /// ```
    pub fn parse(line: &str) -> Self {
        let (cmd, v) = line.split_once(' ').expect("Invalid line");
        let v = v.parse().expect("Could not parse value");
        match cmd {
            "up" => Command::Up(v),
            "down" => Command::Down(v),
            "forward" => Command::Forward(v),
            _ => panic!("Unexpected command"),
        }
    }
}
//end::command[]

//tag::calc_position[]
/// Calculate positions
///
/// # Example
/// ```
/// # use mr_kaffee_2021_02::*;
/// assert_eq!((5, 0), calc_position("forward 5"));
/// assert_eq!((0, 5), calc_position("down 5"));
/// assert_eq!((3, -5), calc_position("up 5\nforward 3"));
/// assert_eq!((0, 0), calc_position(""));
/// ```
pub fn calc_position(input: &str) -> (isize, isize) {
    input
        .lines()
        .map(|line| Command::parse(line))
        .fold((0, 0), |(x, y), cmd| match cmd {
            Command::Up(v) => (x, y - v),
            Command::Down(v) => (x, y + v),
            Command::Forward(v) => (x + v, y),
        })
}
//end::calc_position[]

//tag::calc_position_with_aim[]
/// Calculate positions with aim
///
/// # Example
/// ```
/// # use mr_kaffee_2021_02::*;
/// assert_eq!((5, 0, 0), calc_position_with_aim("forward 5"));
/// assert_eq!((5, 0, 5), calc_position_with_aim("forward 5\ndown 5"));
/// assert_eq!((13, 40, 5), calc_position_with_aim("forward 5\ndown 5\nforward 8"));
/// ```
pub fn calc_position_with_aim(input: &str) -> (isize, isize, isize) {
    input
        .lines()
        .map(|line| Command::parse(line))
        .fold((0, 0, 0), |(x, y, aim), cmd| match cmd {
            Command::Up(v) => (x, y, aim - v),
            Command::Down(v) => (x, y, aim + v),
            Command::Forward(v) => (x + v, y + aim * v, aim),
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
        assert_eq!((15, 10), calc_position(CONTENT));
    }

    #[test]
    fn test_calc_position_with_aim() {
        assert_eq!((15, 60, 10), calc_position_with_aim(CONTENT));
    }
}
