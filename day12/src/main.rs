use std::collections::HashSet;
use union_find_rs::prelude::*;

const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let garden = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let part1 = part1(&garden);
    println!("part1: {part1}");
    assert_eq!(part1, 1456082);

    let part2 = part2(&garden);
    println!("part2: {part2}");
    assert_eq!(part2, 872382);
}

fn part1(garden: &Vec<Vec<u8>>) -> usize {
    let mut visited = HashSet::new();
    let mut price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            let (area, perimeter) =
                compute_area_perimeter(garden, &mut visited, x as i32, y as i32, 0, 0);
            price += area * perimeter;
        }
    }
    price
}

fn compute_area_perimeter(
    garden: &Vec<Vec<u8>>,
    visited: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    mut perimeter: usize,
    mut area: usize,
) -> (usize, usize) {
    if !visited.insert((x, y)) {
        return (area, perimeter);
    }

    let flower = get(garden, x, y).unwrap();

    let neighbors = NEIGHBORS
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(x, y)| get(garden, x, y) == Some(flower))
        .collect::<Vec<_>>();

    area += 1;
    perimeter = perimeter + 4 - neighbors.len();

    for n in neighbors {
        (area, perimeter) = compute_area_perimeter(garden, visited, n.0, n.1, perimeter, area);
    }

    (area, perimeter)
}

fn part2(garden: &Vec<Vec<u8>>) -> usize {
    let mut visited = HashSet::new();
    let mut price = 0;
    for y in 0..garden.len() {
        for x in 0..garden[0].len() {
            let (area, p) = compute_area_sides(
                garden,
                &mut visited,
                x as i32,
                y as i32,
                0,
                DisjointSets::new(),
            );
            let sides = p.into_iter().collect::<Vec<_>>().len();
            price += area * sides;
        }
    }
    price
}

fn compute_area_sides(
    garden: &Vec<Vec<u8>>,
    visited: &mut HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    mut area: usize,
    mut perimeter: DisjointSets<((i32, i32), (i32, i32))>,
) -> (usize, DisjointSets<((i32, i32), (i32, i32))>) {
    if !visited.insert((x, y)) {
        return (area, perimeter);
    }

    let flower = get(garden, x, y).unwrap();

    perimeter = NEIGHBORS
        .into_iter()
        .fold(perimeter, |mut perimeter, (dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            let n_flower = get(garden, nx, ny);
            // dxdy indicates the flower' side that contribute to a side
            let p = ((x, y), (dx, dy));
            // a side is when a neighbors is different than the current flower
            let is_side = n_flower.is_none() || n_flower.unwrap() != flower;
            if is_side {
                let _ = perimeter.make_set(p);
                // merge flowers belonging to the same side
                if dx == 0 {
                    let _ = perimeter.union(&p, &((x - 1, y), (dx, dy)));
                    let _ = perimeter.union(&p, &((x + 1, y), (dx, dy)));
                } else {
                    let _ = perimeter.union(&p, &((x, y - 1), (dx, dy)));
                    let _ = perimeter.union(&p, &((x, y + 1), (dx, dy)));
                }
            }

            perimeter
        });

    area += 1;

    for (nx, ny) in NEIGHBORS
        .into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(x, y)| get(garden, x, y) == Some(flower))
    {
        (area, perimeter) = compute_area_sides(garden, visited, nx, ny, area, perimeter);
    }

    (area, perimeter)
}

fn is_valid(garden: &[Vec<u8>], x: i32, y: i32) -> bool {
    0 <= x && 0 <= y && (x as usize) < garden[0].len() && (y as usize) < garden.len()
}

fn get(garden: &[Vec<u8>], x: i32, y: i32) -> Option<u8> {
    is_valid(garden, x, y).then(|| garden[y as usize][x as usize])
}
