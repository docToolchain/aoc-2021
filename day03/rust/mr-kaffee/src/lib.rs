//tag::parse[]
/// parse input into integers
/// return bit-length of values in addition
pub fn parse(input: &str) -> (Vec<usize>, usize) {
    let len = input.lines().next().expect("No lines").len();
    let values = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).expect("Could not parse line"))
        .collect();
    (values, len)
}
//end::parse[]

//tag::part1[]
/// Count how often bits are set in the values.
///
/// returns vector of counts with first element indicating how often least significant
/// bit is set and last element indicating how often most significant bit is set
pub fn count_all_ones(values: &[usize], len: usize) -> Vec<usize> {
    values.iter().fold(vec![0; len], |counts, value| {
        counts
            .iter()
            .enumerate()
            .map(|(k, one)| *one + ((*value >> k) & 1))
            .collect()
    })
}

/// calculate the gamma and epsilon values
///
/// gamma is the value of all the most common bits
/// epsilon is the value of all the least common bits
///
/// returns ``(gamma, epsilon, gamma * epsilon)``
pub fn calc_gamma_epsilon(values: &[usize], len: usize) -> (usize, usize, usize) {
    let counts = count_all_ones(values, len);
    let gamma = (0..len).fold(0, |gamma, bit| {
        gamma + (((2 * counts[bit] >= values.len()) as usize) << bit)
    });
    let epsilon = !gamma & ((1 << len) - 1);
    (gamma, epsilon, gamma * epsilon)
}
//end::part1[]

//tag::part2[]
/// Count how often specific bit is set in the values.
pub fn count_ones(values: &[usize], bit: usize) -> usize {
    values
        .iter()
        .fold(0, |count, value| count + ((value >> bit) & 1))
}

/// filter out all the values whos given bit is not equal to the most common
/// bit in that position.
///
/// If invert is true, do the opposite and keep all the values whos given bit
/// is not equal to the most common bit in that position.
pub fn filter(values: Vec<usize>, bit: usize, invert: bool) -> Vec<usize> {
    // determin expected value (xor with invert)
    let exp = ((2 * count_ones(&values, bit) >= values.len()) ^ invert) as usize;
    values
        .into_iter()
        .filter(|value| (*value >> bit) & 1 == exp)
        .collect()
}

/// calc oxygen (``invert = false``) or co2 (``invert = true``) ratings
pub fn calc_rating(values: &[usize], len: usize, invert: bool) -> usize {
    let mut values = values.to_vec();
    let mut bit = Some(len - 1);
    while values.len() > 1 && bit.is_some() {
        values = filter(values, bit.unwrap(), invert);
        bit = match bit {
            Some(v) if v > 0 => Some(v - 1),
            _ => None,
        };
    }
    *values.first().expect("No value left")
}

/// calc oxygen and co2 ratings
///
/// returns ``(oxygen, co2, oxygen * co2)``
pub fn calc_ratings(vals: &[usize], len: usize) -> (usize, usize, usize) {
    let oxygen = calc_rating(vals, len, false);
    let co2 = calc_rating(vals, len, true);
    (oxygen, co2, oxygen * co2)
}
//end::part2[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    const EXP_VALUES: &'static [usize] = &[
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    const EXP_LEN: usize = 5;

    #[test]
    fn test_parse() {
        let (vals, len) = parse(CONTENT);
        assert_eq!(EXP_LEN, len);
        assert_eq!(EXP_VALUES, vals);
    }

    #[test]
    fn test_count_all_ones() {
        assert_eq!(vec![5, 7, 8, 5, 7], count_all_ones(EXP_VALUES, EXP_LEN));
    }

    #[test]
    fn test_calc_gamma_epsilon() {
        assert_eq!((22, 9, 22 * 9), calc_gamma_epsilon(EXP_VALUES, EXP_LEN));
    }

    #[test]
    fn test_filter() {
        let values = filter(EXP_VALUES.to_vec(), EXP_LEN - 1, false);
        assert_eq!(
            vec![0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001],
            values
        );

        let values = filter(EXP_VALUES.to_vec(), EXP_LEN - 1, true);
        assert_eq!(vec![0b00100, 0b01111, 0b00111, 0b00010, 0b01010], values);
    }

    #[test]
    fn test_calc_rating() {
        let (values, len) = parse(CONTENT);
        assert_eq!(23, calc_rating(&values, len, false));
        assert_eq!(10, calc_rating(&values, len, true));
    }

    #[test]
    fn test_calc_ratings() {
        let (values, len) = parse(CONTENT);
        assert_eq!((23, 10, 230), calc_ratings(&values, len));
    }
}
//end::tests[]
