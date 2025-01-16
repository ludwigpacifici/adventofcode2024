use fxhash::FxHashSet;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<Vec<_>>>();
    let start = find_guard(&map, b'^').unwrap();

    let visited = walk(&map, start);
    let part1 = visited.len();
    println!("part1: {part1}");
    assert_eq!(part1, 4977);

    let part2 = count_loops(map, &visited, start);
    println!("part2: {part2}");
    assert_eq!(part2, 1729);
}

fn walk(map: &[Vec<u8>], start: (usize, usize)) -> FxHashSet<(i32, i32)> {
    let mut visited = FxHashSet::default();
    let mut position = (start.0 as i32, start.1 as i32);
    let mut dxdy = (0i32, -1);

    loop {
        // invariant: position is valid
        debug_assert!(inside(map, position));
        visited.insert(position);

        let next_position = next(position, dxdy);

        if let Some(c) = get(map, next_position) {
            if c != b'#' {
                position = next_position;
            } else {
                dxdy = turn_right_90(dxdy);
            }
        } else {
            break;
        }
    }
    visited
}

fn count_loops(map: Vec<Vec<u8>>, visited: &FxHashSet<(i32, i32)>, start: (usize, usize)) -> usize {
    visited
        .par_iter()
        .filter(|(x, y)| {
            let (x, y) = (*x as usize, *y as usize);

            // The new obstruction can't be placed at the guard's starting position
            if (x, y) == start {
                return false;
            }

            // it's already a wall, it won't loop
            if map[y][x] == b'#' {
                return false;
            }

            is_loop(&map, start, (x, y))
        })
        .count()
}

// Same as `walk` but record the direction dxdy as well.
fn is_loop(map: &[Vec<u8>], start: (usize, usize), new_obstacle: (usize, usize)) -> bool {
    let new_obstacle = (new_obstacle.0 as i32, new_obstacle.1 as i32);
    let mut visited = FxHashSet::default();
    let mut position = (start.0 as i32, start.1 as i32);
    let mut dxdy = (0i32, -1);

    loop {
        // invariant: position is valid
        debug_assert!(inside(map, position));

        if !visited.insert((position, dxdy)) {
            return true;
        }

        let next_position = next(position, dxdy);

        if let Some(c) = get(map, next_position) {
            // i.e. '.' or '#'
            if c != b'#' && next_position != new_obstacle {
                position = next_position;
            } else {
                dxdy = turn_right_90(dxdy);
            }
        } else {
            break;
        }
    }

    false
}

fn turn_right_90(dxdy: (i32, i32)) -> (i32, i32) {
    match dxdy {
        // Up
        (0, -1) => (1, 0),
        // Down
        (0, 1) => (-1, 0),
        // Left
        (-1, 0) => (0, -1),
        // Right
        (1, 0) => (0, 1),
        unexpected => panic!("Unknown dxdy state to turn right 90 degres: {unexpected:?}"),
    }
}

fn inside(map: &[Vec<u8>], (x, y): (i32, i32)) -> bool {
    0 <= x && 0 <= y && (y as usize) < map.len() && (x as usize) < map[0].len()
}

fn get(map: &[Vec<u8>], (x, y): (i32, i32)) -> Option<u8> {
    if inside(map, (x, y)) {
        Some(map[y as usize][x as usize])
    } else {
        None
    }
}

fn next((x, y): (i32, i32), (dx, dy): (i32, i32)) -> (i32, i32) {
    (x + dx, y + dy)
}

fn find_guard(map: &[Vec<u8>], guard: u8) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == guard {
                return Some((x, y));
            }
        }
    }
    None
}
