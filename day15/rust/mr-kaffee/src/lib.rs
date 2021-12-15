#![cfg_attr(feature = "dijkstra", feature(map_first_last))]
use std::collections::BinaryHeap;

// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_15::*;
/// assert_eq!((vec![0, 1, 2, 3], 2), parse("01\n23"));
/// ```
pub fn parse(content: &str) -> (Vec<usize>, usize) {
    (
        content
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c as usize - '0' as usize)
            .collect(),
        content.lines().next().expect("No data").len(),
    )
}
// end::parse[]

// tag::solve_simple[]
pub fn solve(grid: &[usize], w: usize, n: usize) -> usize {
    let h = grid.len() / w;

    let mut heap = BinaryHeap::new();
    let mut settled = vec![false; grid.len() * n * n];

    heap.push((usize::MAX, 0)); // nodes are tuples (usize::MAX - risk, idx)
    settled[0] = true;

    while let Some((risk, idx)) = heap.pop() {
        if idx == grid.len() * n * n - 1 {
            return usize::MAX - risk; // target reached
        }

        let (x, y) = (idx % (w * n), idx / (w * n));

        for (x_a, y_a) in [
            (x + 1, y),
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
        ] {
            if x_a >= w * n || y_a >= h * n || settled[x_a + y_a * w * n] {
                continue; // out of bounds or visisted
            }

            let risk = risk - ((grid[x_a % w + y_a % h * w] + x_a / w + y_a / h - 1) % 9) - 1;

            settled[x_a + y_a * w * n] = true; // first visit settles
            heap.push((risk, x_a + y_a * w * n));
        }
    }

    panic!("No path found");
}
// end::solve_simple[]

#[cfg(not(feature = "dijkstra"))]
pub fn solution_1(grid: &[usize], width: usize) -> usize {
    solve(grid, width, 1)
}

#[cfg(not(feature = "dijkstra"))]
pub fn solution_2(grid: &[usize], width: usize) -> usize {
    solve(grid, width, 5)
}

#[cfg(feature = "dijkstra")]
pub fn solution_1(grid: &[usize], width: usize) -> usize {
    dijkstra::solve(grid, width, 1)
}

#[cfg(feature = "dijkstra")]
pub fn solution_2(grid: &[usize], width: usize) -> usize {
    dijkstra::solve(grid, width, 5)
}

#[cfg(feature = "dijkstra")]
pub mod dijkstra {
    use std::collections::BTreeSet;

    // tag::solve_dijkstra[]
    pub fn solve(grid: &[usize], w: usize, n: usize) -> usize {
        let h = grid.len() / w;

        let mut heap = BTreeSet::new();
        let mut settled = vec![false; w * h * n * n];
        let mut risks = vec![None; w * h * n * n];

        heap.insert((0, 0)); // nodes are tuples (risk, idx)
        risks[0] = Some(0); // keep track of best risks so far
        settled[0] = true; // flag nodes which are settled

        while let Some((risk, idx)) = heap.pop_first() {
            if idx == grid.len() * n * n - 1 {
                return risk; // target reached
            }
            settled[idx] = true; // no further improvement possible

            let (x, y) = (idx % (w * n), idx / (w * n));
            for (x_a, y_a) in [
                (x + 1, y),
                (x, y + 1),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_sub(1)),
            ] {
                let idx_a = x_a + y_a * w * n;
                if x_a >= w * n || y_a >= h * n || settled[idx_a] {
                    continue; // out of bounds or settled
                }

                let risk_upd =
                    risk + ((grid[x_a % w + y_a % h * w] + x_a / w + y_a / h - 1) % 9) + 1;
                if let Some(risk_cur) = risks[idx_a] {
                    if risk_upd >= risk_cur {
                        continue; // no improvement
                    }
                    // unreachable!("Decrese key can never happen");
                    heap.remove(&(risk_cur, idx_a)); // improved path to node seen previously
                }

                heap.insert((risk_upd, idx_a));
                risks[idx_a] = Some(risk_upd);
            }
        }

        panic!("No path found");
    }
    // end::solve_dijkstra[]
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const TEST_GRID: &'static [usize] = &[
        1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 1, 3, 6, 5, 1, 1, 3, 2, 8,
        3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 1, 3, 1, 9, 1, 2, 8, 1, 3, 7,
        1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 1, 2, 9, 3, 1, 3, 8, 5, 2, 1,
        2, 3, 1, 1, 9, 4, 4, 5, 8, 1,
    ];
    const TEST_WIDTH: usize = 10;

    #[test]
    fn test_parse() {
        let (grid, width) = parse(TEST_CONTENT);
        assert_eq!(TEST_GRID, grid);
        assert_eq!(TEST_WIDTH, width);
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(40, solution_1(&TEST_GRID, TEST_WIDTH))
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(315, solution_2(&TEST_GRID, TEST_WIDTH))
    }
}
// end::tests[]
