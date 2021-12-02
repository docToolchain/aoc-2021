//tag::parse[]
/// parse input into integers
/// return bit-length of values in addition
pub fn parse(input: &str) -> (Vec<usize>, usize) {
    let len = input
        .lines()
        .map(|line| line.len())
        .next()
        .expect("No lines");
    let vals = input
        .lines()
        .map(|line| usize::from_str_radix(line, 2).expect("Could not parse line"))
        .collect();
    (vals, len)
}
//end::parse[]

//tag::part1[]
/// Count how often bits are set in the values.
///
/// The first element in the resulting vector indicates how often the lowest bit is set,
/// the last element indicates how often the highest bit is set
pub fn count_bits(vals: &[usize], len: usize) -> Vec<usize> {
    vals.iter().fold(vec![0; len], |ones, val| {
        ones.iter()
            .enumerate()
            .map(|(pos, one)| *one + ((*val >> pos) & 1))
            .collect()
    })
}

/// calculate the gamma and epsilon values
/// 
/// gamma is the value of all the most common bits
/// epsilon is the value of all the least common bits
/// 
/// returns ``(gamma, epsilon, gamma * epsilon)``
pub fn calc_gamma_epsilon(vals: &[usize], len: usize) -> (usize, usize, usize) {
    let ones = count_bits(&vals, len);
    let gamma = ones.iter().enumerate().fold(0, |acc, (pos, v)| {
        acc + (((*v > vals.len() / 2) as usize) << pos)
    });
    let epsilon = (!gamma) & ((1 << len) - 1);
    (gamma, epsilon, gamma * epsilon)
}
//end::part1[]

//tag::part2[]
/// filter out all the values whos given bit is not equal to the most common 
/// bit in that position.
/// 
/// If invert is true, do the opposite and keep all the values whos given bit 
/// is not equal to the most common bit in that position.
pub fn filter(vals: &[usize], len: usize, bit: usize, invert: bool) -> Vec<usize> {
    let ones = count_bits(vals, len);
    let bit = ones.len() - bit - 1; // take care of bit ordering
    // determin expected value (xor with invert, vals.len() + 1 to ensure 1 in case of tie)
    let exp = ((ones[bit] >= (vals.len() + 1) / 2) as usize) ^ (invert as usize);
    vals.iter()
        .filter(|val| (*val >> bit) & 1 == exp)
        .map(|v| *v)
        .collect()
}

/// calc oxygen (``invert = false``) or co2 (``invert = true``) ratings
pub fn calc_rating(vals: &[usize], len: usize, invert: bool) -> usize {
    let mut vals = filter(vals, len, 0, invert);
    let mut bit = 1;
    while vals.len() > 1 {
        vals = filter(&vals, len, bit, invert);
        bit += 1;
    }
    vals[0]
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

    const EXP_VALS: &'static [usize] = &[
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    const EXP_LEN: usize = 5;

    #[test]
    fn test_parse() {
        let (vals, len) = parse(CONTENT);
        assert_eq!(EXP_LEN, len);
        assert_eq!(EXP_VALS, vals);
    }

    #[test]
    fn test_count_bits() {
        assert_eq!(vec![5, 7, 8, 5, 7], count_bits(EXP_VALS, EXP_LEN));
    }

    #[test]
    fn test_calc_gamma_epsilon() {
        assert_eq!((22, 9, 22 * 9), calc_gamma_epsilon(EXP_VALS, EXP_LEN));
    }

    #[test]
    fn test_filter() {
        let mut vals = EXP_VALS.to_vec();
        let mut bit = 0;
        while vals.len() > 1 {
            vals = filter(&vals, EXP_LEN, bit, false);
            println!("{}, {:?}", bit, vals);
            bit += 1;
        }
        assert_eq!(vec![23], vals);
    }

    #[test]
    fn test_calc_rating() {
        let (vals, len) = parse(CONTENT);
        assert_eq!(23, calc_rating(&vals, len, false));
        assert_eq!(10, calc_rating(&vals, len, true));
    }

    #[test]
    fn test_calc_ratings() {
        let (vals, len) = parse(CONTENT);
        assert_eq!((23, 10, 230), calc_ratings(&vals, len));
    }
}
//end::tests[]