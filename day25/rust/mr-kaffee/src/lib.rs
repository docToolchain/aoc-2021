type C = u8;

// tag::parse[]
pub fn parse(content: &str) -> (Vec<C>, usize) {
    let w = content.lines().next().expect("No content").trim().len();
    let grid = content
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '>' => 1,
            'v' => 2,
            _ => 0,
        })
        .collect();
    (grid, w)
}
// end::parse[]

// tag::solution[]
pub const E: C = 1;
pub const S: C = 2;
pub const X: C = 0;

pub fn step(grid: &[C], w: usize) -> Option<Vec<C>> {
    let mut grid_upd = vec![X; grid.len()];
    let mut moved = false;
    let h = grid.len() / w;

    for x in 0..w {
        let x_upd = (x + 1) % w;
        for y in 0..h {
            let k = x + w * y;
            if grid[k] == E {
                let k_upd = x_upd + w * y;
                if grid[k_upd] == X {
                    grid_upd[k_upd] = E;
                    moved = true;
                } else {
                    grid_upd[k] = E;
                }
            }
        }
    }

    for y in 0..h {
        let y_upd = (y + 1) % h;
        for x in 0..w {
            let k = x + w * y;
            if grid[k] == S {
                let k_upd = x + w * y_upd;
                if grid[k_upd] != S && grid_upd[k_upd] == X {
                    grid_upd[k_upd] = S;
                    moved = true;
                } else {
                    grid_upd[k] = S;
                }
            }
        }
    }

    if moved {
        Some(grid_upd)
    } else {
        None
    }
}

pub fn solution_1(mut grid: Vec<C>, w: usize) -> usize {
    let mut count = 1;
    while let Some(upd) = step(&grid, w) {
        grid = upd;
        count += 1;
    }
    count
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

    const GRID_00: &[C] = &[
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
        assert_eq!(58, solution_1(GRID_00.to_owned(), W));
    }
}
// end::tests[]
