use std::collections::VecDeque;

// tag::parse[]
/// get width of grid and numbers as flat vector
///
/// ```
/// # use mr_kaffee_2021_09::*;
/// assert_eq!((1, vec![0,1]), parse("0\n1"));
/// ```
pub fn parse(content: &str) -> (usize, Vec<usize>) {
    let width = content.lines().next().expect("No lines").len();
    let numbers = content
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| (c as usize - '0' as usize))
        .collect();
    (width, numbers)
}
// end::parse[]

// tag::part1[]
/// find all elements for which all adjacents have higher values, sum their values incremented by 1
pub fn solution_1(width: usize, numbers: &[usize]) -> usize {
    let height = numbers.len() / width;
    (0..numbers.len())
        .map(|idx| (idx % width, idx / width, numbers[idx]))
        .filter(|(x, y, _)| {
            (*x == 0 || numbers[x - 1 + width * y] > numbers[x + width * y])
                && (*x == width - 1 || numbers[x + 1 + width * y] > numbers[x + width * y])
                && (*y == 0 || numbers[x + width * (y - 1)] > numbers[x + width * y])
                && (*y == height - 1 || numbers[x + width * (y + 1)] > numbers[x + width * y])
        })
        .map(|(_, _, number)| number + 1)
        .sum()
}
// end::part1[]

// tag::part2[]
/// get the basin sizes of the basins the given coordinates belong to with a breadth first traversal
pub fn get_basin_size(width: usize, numbers: &[usize], x: usize, y: usize) -> usize {
    // markers for seen coordinates
    let mut seen = vec![false; numbers.len()];
    // queue for breadth first traversal
    let mut queue = VecDeque::new();

    // initialize
    seen[x + width * y] = true;
    queue.push_back((x as isize, y as isize));
    let mut count = 0;

    // height and width of the grid as signed integers
    let height = (numbers.len() / width) as isize;
    let width = width as isize;

    // breadth first traversal
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        count += 1;

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // adjacent coordinates
            let x_a = x + dx;
            let y_a = y + dy;
            if x_a >= 0
                && x_a < width
                && y_a >= 0
                && y_a < height
                && !seen[(x_a + width * y_a) as usize]
                && numbers[(x_a + width * y_a) as usize] != 9
            {
                // if adjacent coordinates in grid, not yet seen and not 9, mark seen and add to queue
                seen[(x_a + width * y_a) as usize] = true;
                queue.push_back((x_a, y_a));
            }
        }
    }

    count
}

/// get the product of the sizes of the (at most) three biggest basins
pub fn solution_2(width: usize, numbers: &[usize]) -> usize {
    // height of the grid
    let height = numbers.len() / width;

    // determine low points
    let lows = (0..numbers.len())
        .map(|idx| (idx % width, idx / width))
        .filter(|(x, y)| {
            (*x == 0 || numbers[x - 1 + width * y] > numbers[x + width * y])
                && (*x == width - 1 || numbers[x + 1 + width * y] > numbers[x + width * y])
                && (*y == 0 || numbers[x + width * (y - 1)] > numbers[x + width * y])
                && (*y == height - 1 || numbers[x + width * (y + 1)] > numbers[x + width * y])
        })
        .collect::<Vec<_>>();

    // determine basin sizes
    let mut basins = lows
        .iter()
        .map(|(x, y)| get_basin_size(width, numbers, *x, *y))
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

    const CONTENT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_parse() {
        let (width, numbers) = parse(CONTENT);
        assert_eq!(10, width);
        assert_eq!(50, numbers.len());
    }

    #[test]
    fn test_solution_1() {
        let (width, numbers) = parse(CONTENT);
        let risk = solution_1(width, &numbers);
        assert_eq!(15, risk);
    }

    #[test]
    fn test_get_basin_size() {
        let (width, numbers) = parse(CONTENT);

        assert_eq!(3, get_basin_size(width, &numbers, 0, 0));
        assert_eq!(9, get_basin_size(width, &numbers, width - 1, 0));
        assert_eq!(14, get_basin_size(width, &numbers, 2, 2));
        assert_eq!(9, get_basin_size(width, &numbers, 6, 4));
    }

    #[test]
    fn test_solution_2() {
        let (width, numbers) = parse(CONTENT);
        assert_eq!(1_134, solution_2(width, &numbers));
    }
}
// end::tests[]
