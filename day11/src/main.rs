use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let part1 = blink_many(&input, 25);
    println!("part1: {part1}");
    assert_eq!(part1, 194482);

    let part2 = blink_many(&input, 75);
    println!("part2: {part2}");
    assert_eq!(part2, 232454623677743);
}

fn blink_many(ns: &[u64], len: usize) -> usize {
    let memo = ns
        .iter()
        .fold(HashMap::new(), |memo, n| blink_one(memo, *n, len));
    ns.iter().filter_map(|n| memo.get(&(*n, len))).sum::<_>()
}

fn blink_one(
    mut memo: HashMap<(u64, usize), usize>,
    n: u64,
    len: usize,
) -> HashMap<(u64, usize), usize> {
    if memo.contains_key(&(n, len)) {
        return memo;
    }

    if len == 0 {
        memo.insert((n, len), 1);
        return memo;
    }

    if n == 0 {
        memo = blink_one(memo, 1, len - 1);
        let result = memo[&(1, len - 1)];
        memo.insert((n, len), result);
        return memo;
    };

    let n_len = if n == 0 { 1 } else { n.ilog10() + 1 };
    if n_len % 2 == 0 {
        let (l, r) = split(n, n_len);
        memo = blink_one(memo, l, len - 1);
        memo = blink_one(memo, r, len - 1);
        let result = memo[&(l, len - 1)] + memo[&(r, len - 1)];
        memo.insert((n, len), result);
        memo
    } else {
        memo = blink_one(memo, n * 2024, len - 1);
        let result = memo[&(n * 2024, len - 1)];
        memo.insert((n, len), result);
        memo
    }
}

fn split(n: u64, len: u32) -> (u64, u64) {
    let l = n / (10u64.pow(len / 2));
    let r = n % (10u64.pow(len / 2));
    (l, r)
}
