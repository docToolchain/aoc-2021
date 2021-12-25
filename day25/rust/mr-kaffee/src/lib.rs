// tag::parse[]
pub fn parse(content: &str) -> (Vec<char>, usize) {
    let w = content.lines().next().expect("No content").trim().len();
    let grid = content.chars().filter(|c| !c.is_whitespace()).collect();
    (grid, w)
}
// end::parse[]

// tag::solution[]
pub const E: char = '>';
pub const S: char = 'v';
pub const X: char = '.';

pub fn step(grid: &[char], w: usize) -> Option<Vec<char>> {
    let mut grid_upd = vec![X; grid.len()];
    let mut moved = false;

    for k in 0..grid.len() {
        if grid[k] == E {
            let k_upd = (k + 1) % w + w * (k / w);
            if grid[k_upd] == X {
                grid_upd[k_upd] = E;
                moved = true;
            } else {
                grid_upd[k] = E;
            }
        }
    }

    for k in 0..grid.len() {
        if grid[k] == S {
            let k_upd = (k + w) % grid.len();
            if grid[k_upd] != S && grid_upd[k_upd] == X {
                grid_upd[k_upd] = S;
                moved = true;
            } else {
                grid_upd[k] = S;
            }
        }
    }

    if moved {
        Some(grid_upd)
    } else {
        None
    }
}

pub fn solution_1(grid: &[char], w: usize) -> usize {
    let mut grid = grid.to_owned();
    let mut count = 0;
    while let Some(upd) = step(&grid, w) {
        grid = upd;
        count += 1;
    }
    count + 1
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_00: &str = "v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>";
    const CONTENT_01: &str = "....>.>v.>
        v.v>.>v.v.
        >v>>..>v..
        >>v>v>.>.v
        .>v.v...v.
        v>>.>vvv..
        ..v...>>..
        vv...>>vv.
        >.v.v..v.v";
    const CONTENT_10: &str = "..>..>>vv.
        v.....>>.v
        ..v.v>>>v>
        v>.>v.>>>.
        ..v>v.vv.v
        .v.>>>.v..
        v.v..>v>..
        ..v...>v.>
        .vv..v>vv.";

    const GRID_00: &[char] = &[
        S, X, X, X, E, E, X, S, S, E, X, S, S, E, E, X, S, S, X, X, E, E, X, E, S, E, X, X, X, S,
        E, E, S, E, E, X, E, X, S, X, S, E, S, X, S, S, X, S, X, X, E, X, E, E, X, X, S, X, X, X,
        X, S, S, X, X, E, X, E, S, X, S, X, S, X, X, E, E, S, X, S, X, X, X, X, S, X, X, S, X, E,
    ];
    const W: usize = 10;

    #[test]
    fn test_parse() {
        let (grid, w) = parse(CONTENT_00);
        assert_eq!(W, w);
        assert_eq!(GRID_00, grid);
    }

    #[test]
    fn test_step() {
        let grid_00 = GRID_00.to_owned();
        let (grid_01, _) = parse(CONTENT_01);
        let (grid_10, _) = parse(CONTENT_10);

        assert_eq!(Some(grid_01), step(&grid_00, W));

        let mut grid = grid_00;
        for _ in 0..10 {
            grid = step(&grid, W).unwrap();
        }
        assert_eq!(grid_10, grid)
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(58, solution_1(GRID_00, W));
    }
}
// end::tests[]
