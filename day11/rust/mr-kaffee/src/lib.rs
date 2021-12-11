// tag::parse[]
/// parse energy levels
/// 
/// # Panics
/// if number of energy levels is not 10 x 10 = 100
pub fn parse(content: &str) -> Vec<usize> {
    let energies = content
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    assert_eq!(
        100,
        energies.len(),
        "Bad length: expected 100, found {}",
        energies.len()
    );
    energies
}
// end::parse[]

// tag::part1[]
/// do an update step on the energy levels
/// return the count of flashes in that step
pub fn step(energies: &mut [usize]) -> usize {
    // flashing stack
    let mut stack = Vec::new();

    // increase all elements by one
    for k in 0..energies.len() {
        energies[k] = (energies[k] + 1) % 10;
        if energies[k] == 0 {
            // flashed -> add to stack
            stack.push(((k % 10) as isize, (k / 10) as isize));
        }
    }

    // depth first traversal to determine all flashes
    let mut flash_count = 0;
    while let Some((x, y)) = stack.pop() {
        flash_count += 1;

        // loop over adjacent entries
        for (x_a, y_a) in [
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
        ] {
            if x_a < 0 || x_a >= 10 || y_a < 0 || y_a >= 10 {
                // out of bounds
                continue;
            }

            // flat index
            let k_a = (x_a + 10 * y_a) as usize;
            if energies[k_a] == 0 {
                // already flashed
                continue;
            }

            // not flashed yet, increment
            energies[k_a] = (energies[k_a] + 1) % 10;
            if energies[k_a] == 0 {
                // flashed -> add to stack
                stack.push((x_a, y_a));
            }
        }
    }

    flash_count
}

/// perform 100 update steps and return total count of flashes
pub fn solution_1(energies: &[usize]) -> usize {
    // work on my own copy of the grid
    let mut energies = energies.to_owned();
    (0..100).map(|_| step(&mut energies)).sum()
}
// end::part1[]

// tag::part2[]
/// perform update steps until all octopuses flash at the same time
/// return the first step when this occurs.
pub fn solution_2(energies: &[usize]) -> usize {
    // work on my own copy of the grid
    let mut energies = energies.to_owned();
    let mut rounds = 1; // one based counting
    while step(&mut energies) < 100 {
        rounds += 1;
    }
    rounds
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const CONTENT_1: &str = "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

    const CONTENT_2: &str = "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

    const CONTENT_10: &str = "0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000";

    #[test]
    fn test_parse() {
        assert_eq!(100, parse(CONTENT).len());
    }

    #[test]
    fn test_step() {
        let mut energies = parse(CONTENT);
        let energies_1 = parse(CONTENT_1);
        let energies_2 = parse(CONTENT_2);
        let energies_10 = parse(CONTENT_10);

        let mut cnt = 0;

        // one step
        cnt += step(&mut energies);
        assert_eq!(0, cnt);
        assert_eq!(energies_1, energies);

        // another step
        cnt += step(&mut energies);
        assert_eq!(35, cnt);
        assert_eq!(energies_2, energies);

        // 8 more steps
        for _ in 2..10 {
            cnt += step(&mut energies);
        }
        assert_eq!(204, cnt);
        assert_eq!(energies_10, energies);
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(1656, solution_1(&parse(CONTENT)));
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(195, solution_2(&parse(CONTENT)));
    }
}
// end::tests[]
