use std::collections::VecDeque;

// tag::parse[]
/// get width of grid and numbers as flat vector
///
/// ```
/// # use mr_kaffee_2021_09::*;
/// assert_eq!((1, vec![0,1]), parse("0\n1"));
/// ```
pub fn parse(content: &str) -> (usize, Vec<usize>) {
    let w = content.lines().next().expect("No lines").len();
    let grid = content
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c as usize - '0' as usize))
        .collect();
    (w, grid)
}
// end::parse[]

// tag::part1[]
/// find all elements for which all adjacents have higher values, sum their values incremented by 1
pub fn solution_1(w: usize, grid: &[usize]) -> usize {
    (0..grid.len())
        .map(|idx| (idx % w, idx / w))
        .filter(|(x, y)| {
            (*x == 0 || grid[x - 1 + w * y] > grid[x + w * y])
                && (*x == w - 1 || grid[x + 1 + w * y] > grid[x + w * y])
                && (*y == 0 || grid[x + w * (y - 1)] > grid[x + w * y])
                && (*y == grid.len() / w - 1 || grid[x + w * (y + 1)] > grid[x + w * y])
        })
        .fold(0, |sum, (x, y)| sum + grid[x + w * y] + 1)
}
// end::part1[]

// tag::part2[]
/// get the basin sizes of the basins the given coordinates belong to with a breadth first traversal
pub fn get_basin_size(w: usize, grid: &[usize], x: usize, y: usize) -> usize {
    let mut unknown = vec![true; grid.len()];
    let mut queue = VecDeque::new();

    unknown[x + w * y] = false;
    queue.push_back((x as isize, y as isize));
    let mut count = 0;

    let h = (grid.len() / w) as isize;
    let w = w as isize;

    // breadth first traversal
    while let Some((x, y)) = queue.pop_front() {
        count += 1;

        for (x_a, y_a) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if ((0..w).contains(&x_a) && (0..h).contains(&y_a))
                && unknown[(x_a + w * y_a) as usize]
                && grid[(x_a + w * y_a) as usize] != 9
            {
                // if adjacent is within grid, is not yet seen and has a value != 9, mark known and add to queue
                unknown[(x_a + w * y_a) as usize] = false;
                queue.push_back((x_a, y_a));
            }
        }
    }

    count
}

/// get the product of the sizes of the (at most) three biggest basins
pub fn solution_2(w: usize, grid: &[usize]) -> usize {
    // determine low points and calculate basin sizes for those
    let mut basins = (0..grid.len())
        .map(|idx| (idx % w, idx / w))
        .filter(|(x, y)| {
            (*x == 0 || grid[x - 1 + w * y] > grid[x + w * y])
                && (*x == w - 1 || grid[x + 1 + w * y] > grid[x + w * y])
                && (*y == 0 || grid[x + w * (y - 1)] > grid[x + w * y])
                && (*y == grid.len() / w - 1 || grid[x + w * (y + 1)] > grid[x + w * y])
        })
        .map(|(x, y)| get_basin_size(w, grid, x, y))
        .collect::<Vec<_>>();

    // sort basin sizes
    basins.sort_unstable();

    // product over the last three elements
    basins.iter().rev().take(3).product()
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";
    const W: usize = 10;
    const GRID: &'static [usize] = &[
        2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9, 2,
        8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
    ];

    #[test]
    fn test_parse() {
        assert_eq!((W, GRID.to_vec()), parse(CONTENT));
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(15, solution_1(W, GRID));
    }

    #[test]
    fn test_get_basin_size() {
        for (exp, (x, y)) in [(3, (1, 0)), (9, (9, 0)), (14, (2, 2)), (9, (6, 4))] {
            assert_eq!(exp, get_basin_size(W, GRID, x, y));
        }
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(1_134, solution_2(W, GRID));
    }
}
// end::tests[]
