use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse(&input);

    let part1 = solve(&input, 0, 100);
    println!("part1: {part1}");
    assert_eq!(part1, 29023);

    let part2 = solve(&input, 10_000_000_000_000, i64::MAX);
    println!("part2: {part2}");
    assert_eq!(part2, 96787395375634);
}

fn solve(input: &[(i64, i64, i64, i64, i64, i64)], offset: i64, max_press: i64) -> i64 {
    input
        .iter()
        .filter_map(|&(ax, ay, bx, by, px, py)| {
            let px = px + offset;
            let py = py + offset;
            let m = (ax * py - ay * px) / (ax * by - bx * ay);
            let n = (px - m * bx) / ax;
            let okx = n * ax + m * bx == px;
            let oky = n * ay + m * by == py;
            (okx && oky && m < max_press && n < max_press).then_some(3 * n + m)
        })
        .sum()
}

// Returns tuples: (ButtonA.x, ButtonA.y, ButtonB.x, ButtonB.y, Prize.x, Prize.y)
fn parse(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let re_button = Regex::new(r"^Button [AB]: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();

    input
        .split("\n\n")
        .map(|machine| {
            let mut it = machine.lines();
            let (_, [ax, ay]) = re_button
                .captures(it.next().unwrap())
                .map(|c| c.extract())
                .unwrap();
            let (_, [bx, by]) = re_button
                .captures(it.next().unwrap())
                .map(|c| c.extract())
                .unwrap();
            let (_, [px, py]) = re_prize
                .captures(it.next().unwrap())
                .map(|c| c.extract())
                .unwrap();
            (
                ax.parse::<i64>().unwrap(),
                ay.parse::<i64>().unwrap(),
                bx.parse::<i64>().unwrap(),
                by.parse::<i64>().unwrap(),
                px.parse::<i64>().unwrap(),
                py.parse::<i64>().unwrap(),
            )
        })
        .collect()
}
