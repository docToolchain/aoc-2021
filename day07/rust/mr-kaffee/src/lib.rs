// tag::parse[]
pub fn parse(content: &str) -> Vec<usize> {
    content
        .trim()
        .split(',')
        .map(|v| v.parse().expect("Could not parse"))
        .collect()
}
// end::parse[]

// tag::select[]
/// Recursive calculation of k-th element.
/// 
/// # Examples
/// 
/// ```
/// # use mr_kaffee_2021_07::*;
/// let data = vec![1,4,7,12,8,13,101];
/// assert_eq!(8, select(&data, data.len() / 2));
/// ```
pub fn select(data: &[usize], k: usize) -> usize {
    if k >= data.len() {
        panic!(
            "Cannot get select rank {} of list of size {}",
            k,
            data.len()
        );
    }

    if data.len() == 1 {
        return data[0];
    }

    let pivot = data[0];

    let lows = data
        .into_iter()
        .filter(|d| d < &&pivot)
        .map(|d| *d)
        .collect::<Vec<_>>();
    let pivots = data
        .into_iter()
        .filter(|d| d == &&pivot)
        .map(|d| *d)
        .collect::<Vec<_>>();
    let highs = data
        .into_iter()
        .filter(|d| d > &&pivot)
        .map(|d| *d)
        .collect::<Vec<_>>();

    return if k < lows.len() {
        select(&lows, k)
    } else if k < lows.len() + pivots.len() {
        pivots[0]
    } else {
        select(&highs, k - lows.len() - pivots.len())
    };
}
// end::select[]

// tag::get_fuel[]
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
// end::get_fuel[]

// tag::cost_functions[]
/// cost function for part 1: identity
pub const COST_LINEAR: fn(usize) -> usize = |distance| distance;

/// cost function for part 2: Euler formula for the sum 1 + ... + distance
pub const COST_INCREASING: fn(usize) -> usize = |distance| distance * (distance + 1) / 2;
// end::cost_functions[]

// tag::optimize[]
/// linear search optimization
pub fn get_optimal_positions_fuel(crabs: &[usize], cost: fn(usize) -> usize) -> usize {
    let pos_min = *crabs.iter().min().expect("No min");
    let pos_max = *crabs.iter().max().expect("No max");
    (pos_min..=pos_max)
        .map(|position| get_fuel(crabs, position, cost))
        .min()
        .expect("No min")
}
// end::optimize[]

// tag::solutions[]
/// solution for part 1
/// 
/// default: linear search, see [get_optimal_positions_fuel] and [COST_LINEAR]
/// 
/// if feature ``direct_solution`` is selected, calculate median as solution, see [select]
pub fn solution_1(crabs: &[usize]) -> usize {
    if cfg!(feature = "direct_solution") {
        // calculate median and comput cost for median
        get_fuel(crabs, select(&crabs, crabs.len() / 2), COST_LINEAR)
    } else {
        // linear search
        get_optimal_positions_fuel(crabs, COST_LINEAR)
    }
}

/// solution for part 1
/// 
/// default: linear search, see [get_optimal_positions_fuel] and [COST_INCREASING]
/// 
/// if feature ``direct_solution`` is selected, calculate mean and search for solution in neighborhood
pub fn solution_2(crabs: &[usize]) -> usize {
    if cfg!(feature = "direct_solution") {
        // calculate mean and cost for mean
        let mean = (crabs.iter().sum::<usize>() + crabs.len() - 1) / crabs.len();
        let mut cost = get_fuel(crabs, mean, COST_INCREASING);

        // search for improvements for increasing positions
        let mut delta = 1;
        loop {
            let cost_upd = get_fuel(crabs, mean + delta, COST_INCREASING);
            if cost_upd >= cost {
                break;
            }

            cost = cost_upd;
            delta += 1;
        }

        // if no improvment found, search for improvements for decreasing positions
        if delta == 1 {
            loop {
                let cost_upd = get_fuel(crabs, mean - delta, COST_INCREASING);
                if cost_upd >= cost {
                    break;
                }

                cost = cost_upd;
                delta += 1;
            }
        }

        cost
    } else {
        // linear search
        get_optimal_positions_fuel(crabs, COST_INCREASING)
    }
}
// end::solutions[]

// tag::tests[]
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
    fn test_solution_1() {
        assert_eq!(37, solution_1(DATA));
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(168, solution_2(DATA));
    }
}
// end::tests[]
