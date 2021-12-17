// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_17::*;
/// assert_eq!((-1, 2, 0, 10), parse("target area: x=-1..2, y=0..10"));
/// ```
pub fn parse(content: &str) -> (isize, isize, isize, isize) {
    let mut parts = content.trim().split(", ");
    let lhs = parts.next().expect("No lhs");
    let rhs = parts.next().expect("No rhs");

    let x_range = lhs.split("=").skip(1).next().expect("No x range");
    let y_range = rhs.split("=").skip(1).next().expect("No y range");

    let mut x_parts = x_range.split("..");
    let mut y_parts = y_range.split("..");

    let x1 = x_parts
        .next()
        .expect("No x1")
        .parse()
        .expect("Cound not parse x1");
    let x2 = x_parts
        .next()
        .expect("No x2")
        .parse()
        .expect("Cound not parse x2");
    let y1 = y_parts
        .next()
        .expect("No y1")
        .parse()
        .expect("Cound not parse y1");
    let y2 = y_parts
        .next()
        .expect("No y2")
        .parse()
        .expect("Cound not parse y2");

    (x1, x2, y1, y2)
}
// end::parse[]

// tag::is_target_reached[]
pub fn is_target_reached(target: (isize, isize, isize, isize), vx: usize, vy: isize) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx as isize;
    let mut vy = vy;

    while (vy > 0 || y >= target.2) && x <= target.1 { // while target can be reached
        x = x + vx;
        y = y + vy;
        vx = if vx > 0 { vx - 1 } else { vx };
        vy = vy - 1;

        if x >= target.0 && x <= target.1 && y >= target.2 && y <= target.3 {
            return true;
        }
    }

    false
}
// end::is_target_reached[]

// tag::part1[]
pub fn solution_1(target: (isize, isize, isize, isize)) -> isize {
    let (x_min, x_max, y_min, y_max) = target;

    assert!(
        x_min > 0 && x_max > x_min && y_max < 0 && y_min < y_max,
        "Solution assumes 0 < x_min < x_max and y_min < y_max < 0"
    );

    // max is reached if x coordinate ends up in target
    // for given initial velocity vy0
    //
    // the max is reached at step k = vy0
    // the max value is vy0 * (vy0 + 1) / 2
    //
    // at step k = 2 * vy0 + 1 the initial value y = 0 is reached again
    // with velocity vy = -vy0 - 1
    //
    // the next step reaches
    //   y = -vy0 - 1
    //
    // the highest max is reached, if y = -vy0 - 1 is just within the target
    // zone, i.e., if y_min = -vy0 - 1 or vy0 = -y_min - 1

    let vy0 = -y_min - 1;
    vy0 * (vy0 + 1) / 2
}
// end::part1[]

// tag::part2[]
pub fn solution_2(target: (isize, isize, isize, isize)) -> usize {
    let (x_min, x_max, y_min, y_max) = target;

    assert!(
        x_min > 0 && x_max > x_min && y_max < 0 && y_min < y_max,
        "Solution assumes 0 < x_min < x_max and y_min < y_max < 0"
    );

    // vx[0] = vx0
    // run for vx0 steps
    // end at vx0 + (vx0 - 1) + ... = vx0 * (vx0 + 1) / 2
    // find smallest absolute vx0 so that target is reached
    let mut vx_min = 0;
    while vx_min * (vx_min + 1) / 2 <= x_min {
        vx_min += 1;
    }

    // if vx0 is above this value, the first step overshoots
    let vx_max = x_max;

    // if below, first step ends below target
    let vy_min = y_min;

    // max from part 1
    let vy_max = -y_min - 1;

    let mut count = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            if is_target_reached(target, vx as usize, vy) {
                count += 1;
            }
        }
    }

    count
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "target area: x=20..30, y=-10..-5";
    const TEST_TARGET: (isize, isize, isize, isize) = (20, 30, -10, -5);

    #[test]
    fn test_parse() {
        assert_eq!(TEST_TARGET, parse(CONTENT));
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(45, solution_1(TEST_TARGET));
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(112, solution_2(TEST_TARGET));
    }

    #[test]
    fn test_is_target_reached() {
        let vs = &[
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];
        for (vx0, vy0) in vs {
            assert!(
                is_target_reached(TEST_TARGET, *vx0, *vy0),
                "Failed at {}, {}",
                vx0,
                vy0
            );
        }
    }
}
// end::tests[]
