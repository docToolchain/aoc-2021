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
/// simulate a given number of days
///
/// the timer values of all fishes are decreased by one. If the timer value is 0,
/// it is reset to 6 and the same amount of fishes with time value 8 is added.
pub fn simulate(mut fishes: Vec<usize>, days: usize) -> Vec<usize> {
    for _ in 0..days {
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
pub fn simulate_and_count(mut fishes: Vec<usize>, days: usize) -> usize {
    fishes = simulate(fishes, days);
    fishes.iter().sum()
}
//end::sol[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &'static [usize] = &[0, 1, 1, 2, 1, 0, 0, 0, 0];

    #[test]
    fn test_simulate() {
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

//tag::direct_solution[]
const CA_80: &'static [usize] = &[1421, 1401, 1191, 1154, 1034, 950, 905, 779, 768];
const CA_256: &'static [usize] = &[
    6703087164, 6206821033, 5617089148, 5217223242, 4726100874, 4368232009, 3989468462, 3649885552,
    3369186778,
];

pub fn solve_direct(fishes: &[usize], days: usize) -> usize {
    fishes
        .iter()
        .zip(
            match days {
                80 => CA_80,
                256 => CA_256,
                _ => panic!("No direct solution for {} days", days),
            }
            .iter(),
        )
        .map(|(counter, factor)| counter * factor)
        .sum()
}

#[cfg(test)]
mod direct_solution_test {
    use super::*;

    const INPUT: &str = "4,1,1,4,1,2,1,4,1,3,4,4,1,5,5,1,3,1,1,1,4,4,3,1,5,3,1,2,5,1,1,5,\
1,1,4,1,1,1,1,2,1,5,3,4,4,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,5,1,1,1,4,1,2,3,5,1,2,2,4,1,4,4,4,\
1,2,5,1,2,1,1,1,1,1,1,4,1,1,4,3,4,2,1,3,1,1,1,3,5,5,4,3,4,1,5,1,1,1,2,2,1,3,1,2,4,1,1,3,3,\
1,3,3,1,1,3,1,5,1,1,3,1,1,1,5,4,1,1,1,1,4,1,1,3,5,4,3,1,1,5,4,1,1,2,5,4,2,1,4,1,1,1,1,3,1,\
1,1,1,4,1,1,1,1,2,4,1,1,1,1,3,1,1,5,1,1,1,1,1,1,4,2,1,3,1,1,1,2,4,2,3,1,4,1,2,1,4,2,1,4,4,\
1,5,1,1,4,4,1,2,2,1,1,1,1,1,1,1,1,1,1,1,4,5,4,1,3,1,3,1,1,1,5,3,5,5,2,2,1,4,1,4,2,1,4,1,2,\
1,1,2,1,1,5,4,2,1,1,1,2,4,1,1,1,1,2,1,1,5,1,1,2,2,5,1,1,1,1,1,2,4,2,3,1,2,1,5,4,5,1,4";

    #[test]
    #[should_panic]
    fn test_direct_fails() {
        solve_direct(&parse(INPUT), 56);
    }

    #[test]
    fn test_direct_80() {
        let fishes = parse(INPUT);
        let sol_direct = solve_direct(&fishes, 80);
        let sol_classic = simulate_and_count(fishes, 80);
        assert_eq!(sol_classic, sol_direct);
    }

    #[test]
    fn test_direct_256() {
        let fishes = parse(INPUT);
        let sol_direct = solve_direct(&fishes, 256);
        let sol_classic = simulate_and_count(fishes, 256);
        assert_eq!(sol_classic, sol_direct);
    }
}
//end::direct_solution[]
