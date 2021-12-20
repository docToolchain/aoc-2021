use std::{cmp, collections::HashSet};

#[derive(Debug, Clone)]
pub struct Image {
    pub data: HashSet<(isize, isize)>,
    pub bbox: (isize, isize, isize, isize),
    pub neg: bool,
}

// tag::parse[]
pub fn parse(content: &str) -> (Vec<bool>, Image) {
    let mut lines = content.lines();
    let algo = lines
        .next()
        .expect("No algo")
        .trim()
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Illegal character: {}", c),
        })
        .collect::<Vec<_>>();

    let mut data = HashSet::new();
    let mut bbox = (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    for (y, line) in lines
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .enumerate()
    {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| c == &'#') {
            data.insert((x as isize, y as isize));
            bbox.0 = cmp::min(bbox.0, x as isize);
            bbox.1 = cmp::min(bbox.1, y as isize);
            bbox.2 = cmp::max(bbox.2, x as isize + 1);
            bbox.3 = cmp::max(bbox.3, y as isize + 1);
        }
    }

    (
        algo,
        Image {
            data,
            bbox,
            neg: false,
        },
    )
}
// end::parse[]

pub fn process(algo: &[bool], image: &Image) -> Image {
    let mut data = HashSet::new();
    let mut bbox = (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    let neg = algo[0] ^ image.neg;

    for y in image.bbox.1 - 1..image.bbox.3 + 1 {
        for x in image.bbox.0 - 1..image.bbox.2 + 1 {
            let mut idx = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    idx =
                        idx << 1 | ((image.neg ^ image.data.contains(&(x + dx, y + dy))) as usize);
                }
            }
            if neg ^ algo[idx] {
                data.insert((x, y));
                bbox.0 = cmp::min(bbox.0, x);
                bbox.1 = cmp::min(bbox.1, y);
                bbox.2 = cmp::max(bbox.2, x + 1);
                bbox.3 = cmp::max(bbox.3, y + 1);
            }
        }
    }

    Image { data, bbox, neg }
}

pub fn steps(algo: &[bool], image: &Image, steps: usize) -> Image {
    let mut image = process(&algo, image);
    for _ in 1..steps {
        image = process(algo, &image);
    }
    image
}

// tag::part1[]
pub fn solution_1(algo: &[bool], image: &Image) -> usize {
    steps(algo, image, 2).data.len()
}
// end::part1[]

// tag::part2[]
pub fn solution_2(algo: &[bool], image: &Image) -> usize {
    steps(algo, image, 50).data.len()
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
\
        #..#.\n\
        #....\n\
        ##..#\n\
        ..#..\n\
        ..###";

    #[test]
    fn test_parse() {
        let (algo, image) = parse(CONTENT);
        assert_eq!(512, algo.len());
        assert_eq!((0, 0, 5, 5), image.bbox);
        assert_eq!(
            HashSet::from([
                (0, 0),
                (3, 0),
                (0, 1),
                (0, 2),
                (1, 2),
                (4, 2),
                (2, 3),
                (2, 4),
                (3, 4),
                (4, 4)
            ]),
            image.data
        );
        assert!(!image.neg);
    }

    #[test]
    fn test_process() {
        let (algo, image) = parse(CONTENT);
        println!("{:?}", image);
        let image = process(&algo, &image);
        println!("{:?}", image);
        let image = process(&algo, &image);
        println!("{:?}", image);
        assert_eq!(35, image.data.len());
    }
}
// end::tests[]
