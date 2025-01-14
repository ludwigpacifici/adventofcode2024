use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (order, updates) = input.split_once("\n\n").unwrap();

    let order: HashMap<u32, HashSet<u32>> = order
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('|').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .fold(HashMap::new(), |mut acc, (l, r)| {
            let set = acc.entry(l).or_default();
            set.insert(r);
            acc
        });

    let updates = updates.lines().map(|l| {
        l.split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    });

    let part1 = updates
        .clone()
        .filter_map(|ns| is_valid(&order, &ns).then_some(mid(&ns)))
        .sum::<u32>();

    println!("part1: {part1}");
    assert_eq!(part1, 4135);

    let part2 = updates
        .clone()
        .filter_map(|ns| {
            (!is_valid(&order, &ns))
                .then_some(sort(&order, ns))
                .map(|ns| mid(&ns))
        })
        .sum::<u32>();

    println!("part2: {part2}");
    assert_eq!(part2, 5285);
}

fn sort(order: &HashMap<u32, HashSet<u32>>, mut ns: Vec<u32>) -> Vec<u32> {
    ns.sort_unstable_by(|l, r| cmp(order, *l, *r).cmp(&true));
    ns
}

fn mid(ns: &[u32]) -> u32 {
    debug_assert!(ns.len() % 2 == 1);
    ns[ns.len() / 2]
}

fn cmp(order: &HashMap<u32, HashSet<u32>>, l: u32, r: u32) -> bool {
    order.get(&l).map(|s| s.contains(&r)).unwrap_or_default()
}

fn is_valid(order: &HashMap<u32, HashSet<u32>>, ns: &[u32]) -> bool {
    ns.windows(2).all(|w| cmp(order, w[0], w[1]))
}
