//tag::parse[]
/// parse fishes
/// 
/// returns an vector of the number of fishes per given timer value
///
/// ```
/// # use mr_kaffee_2021_06::*;
/// assert_eq!(vec![0, 1, 1, 2, 1, 0, 0, 0, 0], parse("3,4,3,1,2"));
/// ```
pub fn parse(content: &str) -> Vec<usize> {
    let mut fishes = vec![0; 9];

    for age in content.trim().split(',') {
        fishes[age.parse::<usize>().expect("Could not parse fish")] += 1;
    }

    fishes
}
//end::parse[]

//tag::sol[]
/// simulate a given number of rounds
/// 
/// the timer values of all fishes are decreased by one. If the timer value is 0, 
/// it is reset to 6 and the same amount of fishes with time value 8 is added.
pub fn simulate(mut fishes: Vec<usize>, rounds: usize) -> Vec<usize> {
    for _ in 0..rounds {
        let new_fishes = fishes[0];
        for k in 1..fishes.len() {
            fishes[k - 1] = fishes[k];
        }
        fishes[6] += new_fishes;
        fishes[8] = new_fishes;
    }
    fishes
}

/// simulate given number of rounds and return sum
pub fn simulate_and_count(mut fishes: Vec<usize>, rounds: usize) -> usize {
    fishes = simulate(fishes, rounds);
    fishes.iter().sum()
}
//end::sol[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &'static [usize] = &[0, 1, 1, 2, 1, 0, 0, 0, 0];

    #[test]
    fn  test_simulate() {
        let mut fishes = DATA.to_vec();
        fishes = simulate(fishes, 1);
        assert_eq!(fishes, parse("2,3,2,0,1"));
        fishes = simulate(fishes, 1);
        assert_eq!(fishes, parse("1,2,1,6,0,8"));
        fishes = simulate(fishes, 1);
        assert_eq!(fishes, parse("0,1,0,5,6,7,8"));
    }

    #[test]
    fn test_simulate_and_count() {
        assert_eq!(5934, simulate_and_count(DATA.to_vec(), 80));
    }
}
//end::tests[]