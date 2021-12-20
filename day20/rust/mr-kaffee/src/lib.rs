use std::cmp;

// tag::image[]
/// structure storing image data
///
/// * ``data`` is a list of all set pixels
/// * ``bbox = (x_min, x_max, y_min, y_max)`` is the bounding box of set pixels
///   with exclusive upper bounds
/// * ``neg`` is a flag indicating whether pixels are inverted (i.e., this is a
///   negative image)
#[derive(Debug, Clone)]
pub struct Image {
    pub data: Vec<(isize, isize)>,
    pub bbox: (isize, isize, isize, isize),
    pub neg: bool,
}
// end::image[]

// tag::parse[]
/// parse content in algorithm (first line) and image (other non blank lines)
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

    let mut data = Vec::new();
    let mut bbox = (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    let neg = false;
    for (y, line) in lines
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .enumerate()
    {
        for (x, _) in line.chars().enumerate().filter(|(_, c)| c == &'#') {
            data.push((x as isize, y as isize));
            bbox.0 = cmp::min(bbox.0, x as isize);
            bbox.1 = cmp::min(bbox.1, y as isize);
            bbox.2 = cmp::max(bbox.2, x as isize + 1);
            bbox.3 = cmp::max(bbox.3, y as isize + 1);
        }
    }

    (algo, Image { data, bbox, neg })
}
// end::parse[]

// tag::update_step[]
/// perform single update step
///
/// create a new image; if ``algo[0]`` is set, the ``neg`` flag of the image will flip
pub fn update_step(algo: &[bool], image: &Image) -> Image {
    let mut data = Vec::with_capacity(image.data.len() * 2);
    let mut bbox = (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    let neg = algo[0] ^ image.neg;

    // it is cheaper to put everything in a grid compared to looking up values repeatedly
    // in a set or map
    // make the grid big enough to not have to check out of bounds in loop later on
    let (x_mn, y_mn, x_mx, y_mx) = image.bbox;
    let x0 = x_mn - 2;
    let y0 = y_mn - 2;
    let w = (x_mx - x_mn) as usize + 4;
    let h = (y_mx - y_mn) as usize + 4;
    let mut grid = vec![false; w * h];
    for (x, y) in &image.data {
        grid[(x - x0) as usize + w * (y - y0) as usize] = true;
    }

    for x in x_mn - 1..x_mx + 1 {
        for y in y_mn - 1..y_mx + 1 {
            let k = (x - x0) as usize + w * (y - y0) as usize;
            let mut idx = 0;
            for j in [
                k - w - 1,
                k - w,
                k - w + 1,
                k - 1,
                k,
                k + 1,
                k + w - 1,
                k + w,
                k + w + 1,
            ] {
                idx = idx << 1 | (image.neg ^ grid[j]) as usize;
            }
            if neg ^ algo[idx] {
                data.push((x, y));
                bbox.0 = cmp::min(bbox.0, x);
                bbox.1 = cmp::min(bbox.1, y);
                bbox.2 = cmp::max(bbox.2, x + 1);
                bbox.3 = cmp::max(bbox.3, y + 1);
            }
        }
    }

    Image { data, bbox, neg }
}
// end::update_step[]

// tag::solution[]
pub fn update_steps(algo: &[bool], image: &Image, steps: usize) -> Image {
    assert!(steps >= 1, "Cannot perform less than one update step");
    let mut image = update_step(&algo, image);
    for _ in 1..steps {
        image = update_step(algo, &image);
    }
    image
}

pub fn solution_1(algo: &[bool], image: &Image) -> usize {
    update_steps(algo, image, 2).data.len()
}
pub fn solution_2(algo: &[bool], image: &Image) -> usize {
    update_steps(algo, image, 50).data.len()
}
// end::solution[]

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
            Vec::from([
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
    fn test_update_steps() {
        let (algo, image) = parse(CONTENT);
        assert_eq!(35, update_steps(&algo, &image, 2).data.len());
        assert_eq!(3_351, update_steps(&algo, &image, 50).data.len());
    }
}
// end::tests[]
