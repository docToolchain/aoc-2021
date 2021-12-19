use std::time::Instant;

// tag::solve[]
pub fn solve() {
    let timer = Instant::now();
    day00::solve();
    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
    day13::solve();
    day14::solve();
    day15::solve();
    day16::solve();
    day17::solve();
    day18::solve();
    day19::solve();
    day20::solve();
    // day21::solve();
    // day22::solve();
    // day23::solve();
    // day24::solve();
    // day25::solve();
    println!("Total time: {:?}", timer.elapsed());
}
// end::solve[]

pub mod day00 {
    use std::time::Instant;

    pub fn solve() {
        let timer = Instant::now();
        println!("Hello, World! Let's go for AoC 2021!");
        println!("-> Solved day 00 in {:?}\n", timer.elapsed());
    }
}

pub mod day01 {
    use mr_kaffee_2021_01::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day01/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 1_374;
    const EXP_2: usize = 1_418;

    pub fn solve() {
        let timer = Instant::now();
        let content = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = count_increase_with_offset(&content, 1);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = count_increase_with_offset(&content, 3);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 01 in {:?}\n", timer.elapsed());
    }
}

pub mod day02 {
    use mr_kaffee_2021_02::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day02/rust/mr-kaffee/input.txt");
    const EXP_1: isize = 2_120_749;
    const EXP_2: isize = 2_138_382_217;

    pub fn solve() {
        let timer = Instant::now();

        let timer_1 = Instant::now();
        let (a1, b1) = calc_position(INPUT);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), a1 * b1);
        assert_eq!(EXP_1, a1 * b1);

        let timer_2 = Instant::now();
        let (a2, b2, _) = calc_position_with_aim(INPUT);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), a2 * b2);
        assert_eq!(EXP_2, a2 * b2);

        println!("-> Solved day 02 in {:?}\n", timer.elapsed());
    }
}

pub mod day03 {
    use mr_kaffee_2021_03::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day03/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 4_006_064;
    const EXP_2: usize = 5_941_884;

    pub fn solve() {
        let timer = Instant::now();
        let (vals, len) = parse(INPUT);

        let timer_1 = Instant::now();
        let (_, _, sol_1) = calc_gamma_epsilon(&vals, len);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let (_, _, sol_2) = calc_ratings(&vals, len);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 03 in {:?}\n", timer.elapsed());
    }
}

pub mod day04 {
    use mr_kaffee_2021_04::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day04/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 10_680;
    const EXP_2: usize = 31_892;

    pub fn solve() {
        let timer = Instant::now();
        let (mut boards, draws) = parse(INPUT);

        let timer_12 = Instant::now();
        let (sol_1, sol_2) = play(&mut boards, &draws);
        println!(
            "Solved part 1 & 2 in {:?}: {:?} / {:?}",
            timer_12.elapsed(),
            sol_1,
            sol_2
        );
        assert_eq!(EXP_1, sol_1);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 04 in {:?}\n", timer.elapsed());
    }
}

pub mod day05 {
    use mr_kaffee_2021_05::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day05/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 6_005;
    const EXP_2: usize = 23_864;

    pub fn solve() {
        let timer = Instant::now();
        let lines = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = count_overlaps(&lines, false);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = count_overlaps(&lines, true);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 05 in {:?}\n", timer.elapsed());
    }
}

pub mod day06 {
    use mr_kaffee_2021_06::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day06/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 376_194;
    const EXP_2: usize = 1_693_022_481_538;

    pub fn solve() {
        let timer = Instant::now();
        let fishes = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = simulate_and_count(fishes.clone(), 80);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = simulate_and_count(fishes.clone(), 256);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 06 in {:?}\n", timer.elapsed());
    }
}

pub mod day07 {
    use mr_kaffee_2021_07::*;
    use std::time::Instant;

    const INPUT: &str = include_str!("../../../../day07/rust/mr-kaffee/input.txt");
    const EXP_1: usize = 337_833;
    const EXP_2: usize = 96_678_050;

    pub fn solve() {
        let timer = Instant::now();
        let crabs = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&crabs);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&crabs);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 07 in {:?}\n", timer.elapsed());
    }
}

pub mod day08 {
    use mr_kaffee_2021_08::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day08/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 397;
    pub const EXP_2: usize = 1_027_422;

    pub fn solve() {
        let timer = Instant::now();
        let data = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&data);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&data);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 08 in {:?}\n", timer.elapsed());
    }
}

pub mod day09 {
    use mr_kaffee_2021_09::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day09/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 594;
    pub const EXP_2: usize = 858_494;

    pub fn solve() {
        let timer = Instant::now();
        let (width, numbers) = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(width, &numbers);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(width, &numbers);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 09 in {:?}\n", timer.elapsed());
    }
}

pub mod day10 {
    use mr_kaffee_2021_10::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day10/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 387_363;
    pub const EXP_2: usize = 4_330_777_059;

    pub fn solve() {
        let timer = Instant::now();
        let data = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&data);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&data);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 10 in {:?}\n", timer.elapsed());
    }
}

pub mod day11 {
    use mr_kaffee_2021_11::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day11/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 1_652;
    pub const EXP_2: usize = 220;

    pub fn solve() {
        let timer = Instant::now();
        let data = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&data);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&data);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 11 in {:?}\n", timer.elapsed());
    }
}

pub mod day12 {
    use mr_kaffee_2021_12::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day12/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 4_241;
    pub const EXP_2: usize = 122_134;

    pub fn solve() {
        let timer = Instant::now();
        let data = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&data);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&data);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 12 in {:?}\n", timer.elapsed());
    }
}

pub mod day13 {
    use mr_kaffee_2021_13::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day13/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 675;
    pub const EXP_2: &str = "#..#.####.#..#.#..#.####.####...##.####\n\
                             #..#....#.#.#..#..#.#....#.......#....#\n\
                             ####...#..##...####.###..###.....#...#.\n\
                             #..#..#...#.#..#..#.#....#.......#..#..\n\
                             #..#.#....#.#..#..#.#....#....#..#.#...\n\
                             #..#.####.#..#.#..#.#....####..##..####\n";

    pub fn solve() {
        let timer = Instant::now();
        let (points, folds) = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&points, &folds);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&points, &folds);
        println!("Solved part 2 in {:?}: \n{}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 13 in {:?}\n", timer.elapsed());
    }
}

pub mod day14 {
    use mr_kaffee_2021_14::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day14/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 2_435;
    pub const EXP_2: usize = 2_587_447_599_164;

    pub fn solve() {
        let timer = Instant::now();
        let (template, rules) = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&template, &rules);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&template, &rules);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 14 in {:?}\n", timer.elapsed());
    }
}

pub mod day15 {
    use mr_kaffee_2021_15::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day15/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 583;
    pub const EXP_2: usize = 2927;

    pub fn solve() {
        let timer = Instant::now();
        let (grid, width) = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&grid, width);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&grid, width);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 15 in {:?}\n", timer.elapsed());
    }
}

pub mod day16 {
    use mr_kaffee_2021_16::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day16/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 883;
    pub const EXP_2: usize = 1_675_198_555_015;

    pub fn solve() {
        let timer = Instant::now();
        let data = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&data);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);

        let timer_2 = Instant::now();
        let sol_2 = solution_2(&data);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);

        println!("-> Solved day 16 in {:?}\n", timer.elapsed());
    }
}

pub mod day17 {
    use mr_kaffee_2021_17::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day17/rust/mr-kaffee/input.txt");
    pub const EXP_1: isize = 2_701;
    pub const EXP_2: usize = 1_070;
    
    pub fn solve() {
        let timer = Instant::now();
        let target = parse(INPUT);
    
        let timer_1 = Instant::now();
        let sol_1 = solution_1(target);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);
    
        let timer_2 = Instant::now();
        let sol_2 = solution_2(target);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);
    
        println!("-> Solved day 17 in {:?}\n", timer.elapsed());
    }
}

pub mod day18 {
    use mr_kaffee_2021_18::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day18/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 4_243;
    pub const EXP_2: usize = 4_701;
    
    pub fn solve() {
        let timer = Instant::now();
        let snailnumbers = parse(INPUT);

        let timer_1 = Instant::now();
        let sol_1 = solution_1(&snailnumbers);
        println!("Solved part 1 in {:?}: {:?}", timer_1.elapsed(), sol_1);
        assert_eq!(EXP_1, sol_1);
    
        let timer_2 = Instant::now();
        let sol_2 = solution_2(&snailnumbers);
        println!("Solved part 2 in {:?}: {:?}", timer_2.elapsed(), sol_2);
        assert_eq!(EXP_2, sol_2);
    
        println!("-> Solved day 18 in {:?}\n", timer.elapsed());
    }
}

pub mod day19 {
    use mr_kaffee_2021_19::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day19/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 392;
    pub const EXP_2: usize = 13_332;
    
    pub fn solve() {
        let timer = Instant::now();

        let data = parse(INPUT);

        let timer_1_2 = Instant::now();
        let (sol_1, sol_2) = solution_1_2(&data);
        println!("Solved part 1 & 2 in {:?}: {:?} / {:?}", timer_1_2.elapsed(), sol_1, sol_2);
        assert_eq!(EXP_1, sol_1);
        assert_eq!(EXP_2, sol_2);
    
        println!("-> Solved day 19 in {:?}\n", timer.elapsed());
    }
}

pub mod day20 {
    // use mr_kaffee_2021_20::*;
    use std::time::Instant;

    pub const INPUT: &str = include_str!("../../../../day20/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 20 not yet implemented");

        println!("-> Solved day 20 in {:?}\n", timer.elapsed());
    }
}

pub mod day21 {
    // use mr_kaffee_2021_21::*;
    use std::time::Instant;

    // pub const INPUT: &str = include_str!("../../../../day21/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 21 not yet implemented");

        println!("-> Solved day 21 in {:?}\n", timer.elapsed());
    }
}

pub mod day22 {
    // use mr_kaffee_2021_22::*;
    use std::time::Instant;

    // pub const INPUT: &str = include_str!("../../../../day22/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 22 not yet implemented");

        println!("-> Solved day 22 in {:?}\n", timer.elapsed());
    }
}

pub mod day23 {
    // use mr_kaffee_2021_23::*;
    use std::time::Instant;

    // pub const INPUT: &str = include_str!("../../../../day23/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 23 not yet implemented");

        println!("-> Solved day 23 in {:?}\n", timer.elapsed());
    }
}

pub mod day24 {
    // use mr_kaffee_2021_24::*;
    use std::time::Instant;

    // pub const INPUT: &str = include_str!("../../../../day24/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 24 not yet implemented");

        println!("-> Solved day 24 in {:?}\n", timer.elapsed());
    }
}

pub mod day25 {
    // use mr_kaffee_2021_25::*;
    use std::time::Instant;

    // pub const INPUT: &str = include_str!("../../../../day25/rust/mr-kaffee/input.txt");
    pub const EXP_1: usize = 0;
    pub const EXP_2: usize = 0;

    pub fn solve() {
        let timer = Instant::now();

        assert!(false, "Day 25 not yet implemented");

        println!("-> Solved day 25 in {:?}\n", timer.elapsed());
    }
}
