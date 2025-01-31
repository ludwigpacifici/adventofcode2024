use regex::Regex;
use std::collections::HashSet;

const X_LEN: usize = 101;
const Y_LEN: usize = 103;
const QUADRANTS: [((usize, usize), (usize, usize)); 4] = [
    ((0, 0), (X_LEN / 2, Y_LEN / 2)),
    ((X_LEN / 2 + 1, 0), (X_LEN, Y_LEN / 2)),
    ((X_LEN / 2 + 1, Y_LEN / 2 + 1), (X_LEN, Y_LEN)),
    ((0, Y_LEN / 2 + 1), (X_LEN / 2, Y_LEN)),
];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"^p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();
    let robots = input
        .trim()
        .lines()
        .map(|l| {
            let (_, [px, py, vx, vy]) = re.captures(l).map(|c| c.extract()).unwrap();
            let p = (px.parse::<usize>().unwrap(), py.parse::<usize>().unwrap());
            let v = (
                vx.parse::<i32>().unwrap().rem_euclid(X_LEN as i32) as usize,
                vy.parse::<i32>().unwrap().rem_euclid(Y_LEN as i32) as usize,
            );
            (p, v)
        })
        .collect::<Vec<_>>();

    let part1 = robots
        .iter()
        .filter_map(|&(p, v)| find_quandrant_i(move_n(p, v, 100)))
        .fold([0; QUADRANTS.len()], |mut acc, qi| {
            acc[qi] += 1;
            acc
        })
        .iter()
        .product::<usize>();
    println!("part1: {part1}");
    assert_eq!(part1, 229980828);

    // I assumed the tree looked like, for X_LEN = 11, Y_LEN = 7 and
    // 12 robots (https://adventofcode.com/2015). Therefore all robots
    // have a unique position.
    //
    // .....*.....
    // ....*.*....
    // ...*...*...
    // ..*.....*..
    // .*.......*.
    // *.........*
    // .....*.....

    let mut part2 = 0;
    while !all_unique(&robots, part2) {
        part2 += 1;
    }
    println!("part2: {part2}");
    assert_eq!(part2, 7132);
}

fn all_unique(robots: &[((usize, usize), (usize, usize))], n: usize) -> bool {
    robots
        .iter()
        .map(|&(p, v)| move_n(p, v, n))
        .collect::<HashSet<_>>()
        .len()
        == robots.len()
}

fn move_n((px, py): (usize, usize), (vx, vy): (usize, usize), n: usize) -> (usize, usize) {
    let px = (px + (vx * n % X_LEN)) % X_LEN;
    let py = (py + (vy * n % Y_LEN)) % Y_LEN;
    (px, py)
}

fn find_quandrant_i((px, py): (usize, usize)) -> Option<usize> {
    QUADRANTS
        .iter()
        .position(|&((qx0, qy0), (qx1, qy1))| qx0 <= px && px < qx1 && qy0 <= py && py < qy1)
}
