use std::collections::{HashSet, VecDeque};

const BOX: u8 = b'O';
const BOX_LEFT: u8 = b'[';
const BOX_RIGHT: u8 = b']';
const EMPTY: u8 = b'.';
const START: u8 = b'@';
const WALL: u8 = b'#';

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (maze, directions) = input.split_once("\n\n").unwrap();
    let mut maze = maze
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let directions = directions.lines().flat_map(|l| l.chars().map(dxdy));

    let start = find_start(&maze);
    maze[start[1]][start[0]] = EMPTY; // mark the start as empty

    let part1 = part1(directions.clone(), maze.clone(), start);
    println!("part1 : {part1}");
    assert_eq!(part1, 1412971);

    let maze = wide_maze(&maze);
    let start = [2 * start[0], start[1]];
    let part2 = part2(directions, maze, start);
    println!("part2 : {part2}");
    assert_eq!(part2, 1429299);
}

fn part2(
    directions: impl Iterator<Item = [i32; 2]>,
    mut maze: Vec<Vec<u8>>,
    [mut x, mut y]: [usize; 2],
) -> usize {
    for dxdy in directions {
        let [nx, ny] = next(x, y, dxdy);
        match maze[ny][nx] {
            EMPTY => [x, y] = [nx, ny],
            WALL => continue,
            _ => {
                if dxdy[1] == 0 {
                    if let Some([empty_x, empty_y]) = position(&maze, nx, ny, dxdy, EMPTY) {
                        maze = rotate(maze, empty_x, empty_y, nx, ny, dxdy_rev(dxdy));
                        [x, y] = [nx, ny];
                    }
                } else {
                    let mut boxes = adjacent_boxes(&maze, [nx, ny], dxdy)
                        .into_iter()
                        .collect::<Vec<_>>();
                    boxes.sort_unstable_by_key(|&(_, by)| by.abs_diff(y));

                    for &(bx, by) in boxes.iter().rev() {
                        let [nbx, nby] = next(bx, by, dxdy);
                        let tmp = maze[nby][nbx];
                        maze[nby][nbx] = maze[by][bx];
                        maze[by][bx] = tmp;
                    }

                    if !boxes.is_empty() {
                        [x, y] = [nx, ny];
                    }
                }
            }
        };
    }
    answer(&maze)
}

fn adjacent_boxes(maze: &[Vec<u8>], start: [usize; 2], dxdy: [i32; 2]) -> HashSet<(usize, usize)> {
    let [sx, sy] = start;

    debug_assert!(maze[sy][sx] == BOX_LEFT || maze[sy][sx] == BOX_RIGHT);
    debug_assert!(dxdy[0] == 0);

    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    let mut adjacent_boxes = HashSet::new();
    q.push_back(start);
    while let Some([x, y]) = q.pop_front() {
        if !seen.insert([x, y]) {
            continue;
        }

        if maze[y][x] == EMPTY {
            continue;
        } else if maze[y][x] == WALL {
            // There is a wall at the top or bottom. Nothing can move.
            return HashSet::new();
        } else {
            adjacent_boxes.insert((x, y));
        }

        // Check the other side of the box, and in the dy direction
        [dxdy, [if maze[y][x] == BOX_LEFT { 1 } else { -1 }, 0]]
            .iter()
            .map(|dxdy| next(x, y, *dxdy))
            .for_each(|nxy| {
                q.push_back(nxy);
            });
    }
    adjacent_boxes
}

fn part1(
    directions: impl Iterator<Item = [i32; 2]>,
    mut maze: Vec<Vec<u8>>,
    [mut x, mut y]: [usize; 2],
) -> usize {
    for dxdy in directions {
        let [nx, ny] = next(x, y, dxdy);
        if let Some([empty_x, empty_y]) = position(&maze, nx, ny, dxdy, EMPTY) {
            maze = rotate(maze, empty_x, empty_y, nx, ny, dxdy_rev(dxdy));
            [x, y] = [nx, ny];
        }
    }
    answer(&maze)
}

fn answer(maze: &[Vec<u8>]) -> usize {
    maze.iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(|(x, b)| (*b == BOX_LEFT || *b == BOX).then_some(x + y * 100))
                .sum::<usize>()
        })
        .sum()
}

fn position(
    maze: &[Vec<u8>],
    mut x: usize,
    mut y: usize,
    dxdy: [i32; 2],
    t: u8,
) -> Option<[usize; 2]> {
    loop {
        // no need to check for out of bounds, the perimeter is a
        // wall.
        if maze[y][x] == WALL {
            return None;
        } else if maze[y][x] == t {
            return Some([x, y]);
        }
        [x, y] = next(x, y, dxdy);
    }
}

fn rotate(
    mut maze: Vec<Vec<u8>>,
    mut from_x: usize,
    mut from_y: usize,
    to_x: usize,
    to_y: usize,
    dxdy: [i32; 2],
) -> Vec<Vec<u8>> {
    let tmp = maze[from_y][from_x];
    debug_assert!(tmp == EMPTY);

    while [from_x, from_y] != [to_x, to_y] {
        let [nx, ny] = next(from_x, from_y, dxdy);
        maze[from_y][from_x] = maze[ny][nx];
        [from_x, from_y] = [nx, ny];
    }
    maze[to_y][to_x] = tmp;
    maze
}

fn next(x: usize, y: usize, [dx, dy]: [i32; 2]) -> [usize; 2] {
    [((x as i32) + dx) as usize, ((y as i32) + dy) as usize]
}

fn dxdy(c: char) -> [i32; 2] {
    match c {
        '^' => [0, -1],
        'v' => [0, 1],
        '<' => [-1, 0],
        '>' => [1, 0],
        c => panic!("Unknown direction: {c}"),
    }
}

fn dxdy_rev([dx, dy]: [i32; 2]) -> [i32; 2] {
    [-dx, -dy]
}

fn find_start(maze: &[Vec<u8>]) -> [usize; 2] {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            if maze[y][x] == START {
                return [x, y];
            }
        }
    }
    panic!("start '@' not found")
}

fn wide_maze(maze: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut m = vec![Vec::with_capacity(2 * maze[0].len()); maze.len()];
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            match maze[y][x] {
                BOX => {
                    m[y].push(b'[');
                    m[y].push(b']');
                }
                other => {
                    m[y].push(other);
                    m[y].push(other);
                }
            }
        }
    }
    m
}
