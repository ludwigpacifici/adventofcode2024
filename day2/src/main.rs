fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });

    let p1 = input.clone().filter(|ns| part1(ns)).count();
    println!("part1: {p1}");
    assert_eq!(p1, 490);

    let p2 = input.filter(|ns| part2(ns)).count();
    println!("part2: {p2}");
    assert_eq!(p2, 536);
}

fn part1(ns: &[i32]) -> bool {
    if ns.len() == 1 {
        true
    } else {
        let sign = ns[0] < ns[1];
        for w in ns.windows(2) {
            if !is_valid(sign, w[0], w[1]) {
                return false;
            }
        }
        true
    }
}

fn part2(ns: &[i32]) -> bool {
    if part1(ns) {
        return true;
    }

    for i in 0..ns.len() {
        let mut ns = ns.to_vec();
        ns.remove(i);
        if part1(&ns) {
            return true;
        }
    }

    false
}

fn is_valid(sign: bool, f: i32, s: i32) -> bool {
    let is_sign_valid = (f < s) == sign;
    let d = s.abs_diff(f);
    let is_diff_valid = (1..=3).contains(&d);

    is_sign_valid && is_diff_valid
}

#[cfg(test)]
mod test {
    use super::part2;
    #[test]
    fn part2_tests() {
        assert_eq!(part2(vec![1, 1, 1, 1]), false);
        assert_eq!(part2(vec![7, 6, 4, 2, 1]), true);
        assert_eq!(part2(vec![1, 2, 7, 8, 9]), false);
        assert_eq!(part2(vec![9, 7, 6, 2, 1]), false);
        assert_eq!(part2(vec![1, 3, 2, 4, 5]), true);
        assert_eq!(part2(vec![8, 6, 4, 4, 1]), true);
        assert_eq!(part2(vec![1, 3, 6, 7, 9]), true);
        assert_eq!(part2(vec![1, -1, 1, -1]), false);
        assert_eq!(part2(vec![63, 60, 60, 58, 55, 52, 54]), false);
        assert_eq!(part2(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(part2(vec![2, 1, 2, 3, 4, 5]), true);
        assert_eq!(part2(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(part2(vec![56, 53, 55, 56, 58, 60]), true);
        assert_eq!(part2(vec![56, 53, 55, 50, 48, 45]), true);
    }
}
