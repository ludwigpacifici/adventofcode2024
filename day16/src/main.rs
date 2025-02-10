use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut maze = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let start = find(&maze, b'S').unwrap();
    let end = find(&maze, b'E').unwrap();

    let dist = dijkstra(&mut maze, start, end, (1, 0));
    let part1 = *dist
        .iter()
        .filter_map(|((xy, _), v)| (*xy == end).then_some(v))
        .min()
        .unwrap();
    println!("part1: {part1}");
    assert_eq!(part1, 109496);

    let part2 = part2(&maze, dist, start, end, part1);
    println!("part2: {part2}");
    assert_eq!(part2, 551);
}

type Position = (usize, usize);
type Dxdy = (i32, i32);
type Distances = HashMap<(Position, Dxdy), u64>;

fn dijkstra(maze: &mut [Vec<u8>], (x, y): Position, end: Position, dxdy: Dxdy) -> Distances {
    let mut dist = HashMap::new();
    dist.insert(((x, y), dxdy), 0);
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(Item {
        xy: (x, y),
        dxdy,
        score: 0,
    }));
    let mut prev = HashMap::new();

    while let Some(Reverse(i)) = pq.pop() {
        if i.xy == end {
            return dist;
        }

        for (nxy, ndxdy, w) in neighors(i.xy, i.dxdy)
            .into_iter()
            .filter(|&((nx, ny), _, _)| maze[ny][nx] != b'#')
        {
            let alt = dist.get(&(i.xy, i.dxdy)).unwrap() + w;
            if alt < *dist.get(&(nxy, ndxdy)).unwrap_or(&u64::MAX) {
                prev.insert(nxy, Some(i.xy));
                dist.insert((nxy, ndxdy), alt);
                pq.push(Reverse(Item {
                    xy: nxy,
                    dxdy: ndxdy,
                    score: alt,
                }));
            }
        }
    }

    dist
}

fn part2(maze: &[Vec<u8>], dist: Distances, start: Position, end: Position, score: u64) -> usize {
    let end_dxdys = dist
        .iter()
        .filter_map(|(&(xy, dxdy), &s)| (xy == end && s == score).then_some(opposite(dxdy)))
        .collect::<Vec<_>>();
    let mut q = VecDeque::new();
    // walk backward
    for end_dxdy in end_dxdys {
        q.push_back((end, end_dxdy, score, HashSet::new()));
    }

    let mut part2 = HashSet::new();
    while let Some((xy, dxdy, s, mut seen)) = q.pop_front() {
        seen.insert(xy);

        if s == 0 && xy != start {
            continue;
        } else if (s == 0 || s == 1000) && xy == start {
            part2.extend(seen);
            continue;
        }

        for (nxy, ndxdy, w) in neighors(xy, dxdy).into_iter().filter(|&((nx, ny), _, _)| {
            maze[ny][nx] != b'#'
                && !seen.contains(&(nx, ny))
                && dist
                    .iter()
                    .any(|((dxy, _), ds)| (nx, ny) == *dxy && (ds + 1 == s || ds + 1_001 == s))
        }) {
            q.push_back((nxy, ndxdy, s - w, seen.clone()));
        }
    }

    part2.len()
}

fn opposite((dx, dy): Dxdy) -> Dxdy {
    (-dx, -dy)
}

#[derive(Debug, Eq, PartialEq)]
struct Item {
    xy: Position,
    dxdy: Dxdy,
    score: u64,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// 1st: forward
// 2nd and 3rd: orthogonal position
fn neighors((x, y): Position, (dx, dy): Dxdy) -> [(Position, Dxdy, u64); 3] {
    let (x, y) = (x as i32, y as i32);
    let forward = (((x + dx) as usize, (y + dy) as usize), (dx, dy), 1);
    let orthogonal_1 = (((x + dy) as usize, (y + dx) as usize), (dy, dx), 1_001);
    let orthogonal_2 = (((x - dy) as usize, (y - dx) as usize), (-dy, -dx), 1_001);
    [forward, orthogonal_1, orthogonal_2]
}

fn find(maze: &[Vec<u8>], b: u8) -> Option<Position> {
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == b {
                return Some((x, y));
            }
        }
    }
    None
}
