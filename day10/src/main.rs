use std::collections::HashSet;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let trailheads = map.iter().enumerate().flat_map(|(y, l)| {
        l.iter()
            .enumerate()
            .filter_map(|(x, b)| (*b == b'0').then_some(x))
            .map(move |x| (x, y))
    });

    let part1 = trailheads
        .clone()
        .map(|(x, y)| walk(&map, HashSet::new(), (x as i32, y as i32)).len())
        .sum::<usize>();
    println!("part1: {part1}");
    assert_eq!(part1, 552);

    let part2 = trailheads
        .map(|(x, y)| walk_2(&map, (x as i32, y as i32)))
        .sum::<usize>();
    println!("part2: {part2}");
    assert_eq!(part2, 1225);
}

fn walk(
    map: &[Vec<u8>],
    mut nines: HashSet<(i32, i32)>,
    (x, y): (i32, i32),
) -> HashSet<(i32, i32)> {
    // invariant: caller passed valid coordinates
    debug_assert!(is_valid(map[0].len(), map.len(), (x, y)));

    let altitude = map[y as usize][x as usize];
    if altitude == b'9' {
        nines.insert((x, y));
        nines
    } else {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let next_xy = (x + dx, y + dy);
                is_valid(map[0].len(), map.len(), next_xy)
                    .then_some(next_xy)
                    .filter(|next_xy| {
                        let next_altitude = map[next_xy.1 as usize][next_xy.0 as usize];
                        is_next_altitude(altitude, next_altitude)
                    })
            })
            .fold(nines, |acc, next| walk(map, acc, next))
    }
}

fn walk_2(map: &[Vec<u8>], (x, y): (i32, i32)) -> usize {
    // invariant: caller passed valid coordinates
    debug_assert!(is_valid(map[0].len(), map.len(), (x, y)));

    let altitude = map[y as usize][x as usize];
    if altitude == b'9' {
        1
    } else {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|(dx, dy)| {
                let next_xy = (x + dx, y + dy);
                is_valid(map[0].len(), map.len(), next_xy)
                    .then_some(next_xy)
                    .filter(|next_xy| {
                        let next_altitude = map[next_xy.1 as usize][next_xy.0 as usize];
                        is_next_altitude(altitude, next_altitude)
                    })
            })
            .map(|next_xy| walk_2(map, next_xy))
            .sum()
    }
}

fn is_valid(x_len: usize, y_len: usize, (x, y): (i32, i32)) -> bool {
    0 <= x && 0 <= y && (x as usize) < x_len && (y as usize) < y_len
}

fn is_next_altitude(current: u8, next: u8) -> bool {
    // (b'0'..b'9') with '9' excluded.
    (48..57).contains(&current) && current + 1 == next
}
