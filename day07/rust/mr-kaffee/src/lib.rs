//tag::parse[]
pub fn parse(content: &str) -> Vec<usize> {
    content
        .trim()
        .split(',')
        .map(|v| v.parse().expect("Could not parse"))
        .collect()
}
//end::parse[]

//tag::get_fuel[]
pub fn get_fuel(crabs: &[usize], position: usize, cost: fn(usize) -> usize) -> usize {
    crabs
        .iter()
        .map(|crab| {
            cost(if crab > &position {
                crab - position
            } else {
                position - crab
            })
        })
        .sum()
}
//end::get_fuel[]

//tag::cost_functions[]
pub const COST_LINEAR: fn(usize) -> usize = |distance| distance;
pub const COST_INCREASING: fn(usize) -> usize = |distance| distance * (distance + 1) / 2;
//end::cost_functions[]

//tag::optimize[]
pub fn get_optimal_positions_fuel(crabs: &[usize], cost: fn(usize) -> usize) -> usize {
    let pos_min = *crabs.iter().min().expect("No min");
    let pos_max = *crabs.iter().max().expect("No max");
    (pos_min..=pos_max)
        .map(|position| get_fuel(crabs, position, cost))
        .min()
        .expect("No min")
}
//end::optimize[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "16,1,2,0,4,2,7,1,2,14";
    const DATA: &'static [usize] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_parse() {
        assert_eq!(DATA, parse(CONTENT));
    }

    #[test]
    fn test_get_fuel() {
        assert_eq!(37, get_fuel(DATA, 2, COST_LINEAR));
        assert_eq!(41, get_fuel(DATA, 1, COST_LINEAR));
        assert_eq!(39, get_fuel(DATA, 3, COST_LINEAR));
    }

    #[test]
    fn test_get_optimal_positions_fuel() {
        assert_eq!(37, get_optimal_positions_fuel(DATA, COST_LINEAR));
    }

    #[test]
    fn test_get_optimal_positions_fuel_2() {
        assert_eq!(168, get_optimal_positions_fuel(DATA, COST_INCREASING));
    }
}
//end::tests[]
