use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let mut all_nodes: HashMap<u8, Vec<(_, _)>> = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let m = map[y][x];
            if m != b'.' {
                let ns = all_nodes.entry(m).or_default();
                ns.push((x as i32, y as i32));
            }
        }
    }

    let all_antinodes = find_antinodes(&all_nodes, &map, 1);
    let part1 = all_antinodes.len();
    println!("part1: {part1}");
    assert_eq!(part1, 323);

    let mut all_antinodes = find_antinodes(&all_nodes, &map, map.len() as i32);
    // Antennas now create many antinodes
    for nodes in all_nodes.values() {
        all_antinodes = nodes.iter().fold(all_antinodes, |mut acc, xy| {
            acc.insert(*xy);
            acc
        });
    }
    let part2 = all_antinodes.len();
    println!("part2: {part2}");
    assert_eq!(part2, 1077);
}

fn find_antinodes(
    all_nodes: &HashMap<u8, Vec<(i32, i32)>>,
    map: &[Vec<u8>],
    max_range: i32,
) -> HashSet<(i32, i32)> {
    let mut all_antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (node_val, nodes) in all_nodes {
        for i in 0..nodes.len() - 1 {
            for j in i + 1..nodes.len() {
                let (xi, yi) = nodes[i];
                let (xj, yj) = nodes[j];
                let (dx, dy) = (xi - xj, yi - yj);

                all_antinodes = (-max_range..=max_range)
                    .map(|n| (xi + n * dx, yi + n * dy))
                    .chain((-max_range..=max_range).map(|n| (xj + n * dx, yj + n * dy)))
                    .filter(|&(x, y)| {
                        0 <= x
                            && 0 <= y
                            && (x as usize) < map[0].len()
                            && (y as usize) < map.len()
                            && map[y as usize][x as usize] != *node_val
                    })
                    .fold(all_antinodes, |mut all_antinodes, (x, y)| {
                        all_antinodes.insert((x, y));
                        all_antinodes
                    });
            }
        }
    }
    all_antinodes
}
