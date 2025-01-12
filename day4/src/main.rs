fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let part1 = count_all(&input, xmas_count);
    println!("part1: {part1}");
    assert_eq!(part1, 2496);

    let part2 = count_all(&input, x_mas_count);
    println!("part2: {part2}");
    assert_eq!(part2, 1967);
}

fn count_all(input: &[&[u8]], f: impl Fn(&[&[u8]], usize, usize) -> usize) -> usize {
    let y_len = input.len();
    let x_len = input[0].len();
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            count += f(input, x, y);
        }
    }
    count
}

fn x_mas_count(input: &[&[u8]], x: usize, y: usize) -> usize {
    let y_len = input.len();
    let x_len = input[0].len();

    (x + 2 < x_len
        && y + 2 < y_len
        && input[y + 1][x + 1] == b'A'
        && ((input[y][x] == b'M' && input[y + 2][x + 2] == b'S')
            || (input[y][x] == b'S' && input[y + 2][x + 2] == b'M'))
        && ((input[y + 2][x] == b'M' && input[y][x + 2] == b'S')
            || (input[y + 2][x] == b'S' && input[y][x + 2] == b'M'))) as usize
}

fn xmas_count(input: &[&[u8]], x: usize, y: usize) -> usize {
    let y_len = input.len();
    let x_len = input[0].len();

    [
        // forward horizontal
        x + 3 < x_len
            && input[y][x] == b'X'
            && input[y][x + 1] == b'M'
            && input[y][x + 2] == b'A'
            && input[y][x + 3] == b'S',
        // backward horizontal
        x + 3 < x_len
            && input[y][x] == b'S'
            && input[y][x + 1] == b'A'
            && input[y][x + 2] == b'M'
            && input[y][x + 3] == b'X',
        // forward vertical
        y + 3 < y_len
            && input[y][x] == b'X'
            && input[y + 1][x] == b'M'
            && input[y + 2][x] == b'A'
            && input[y + 3][x] == b'S',
        // backward vertical
        y + 3 < y_len
            && input[y][x] == b'S'
            && input[y + 1][x] == b'A'
            && input[y + 2][x] == b'M'
            && input[y + 3][x] == b'X',
        // forward diagonal
        x + 3 < x_len
            && y + 3 < y_len
            && input[y + 3][x] == b'X'
            && input[y + 2][x + 1] == b'M'
            && input[y + 1][x + 2] == b'A'
            && input[y][x + 3] == b'S',
        // backward diagonal
        x + 3 < x_len
            && y + 3 < y_len
            && input[y + 3][x] == b'S'
            && input[y + 2][x + 1] == b'A'
            && input[y + 1][x + 2] == b'M'
            && input[y][x + 3] == b'X',
        // forward other diagonal
        x + 3 < x_len
            && y + 3 < y_len
            && input[y][x] == b'X'
            && input[y + 1][x + 1] == b'M'
            && input[y + 2][x + 2] == b'A'
            && input[y + 3][x + 3] == b'S',
        // backward other diagonal
        x + 3 < x_len
            && y + 3 < y_len
            && input[y][x] == b'S'
            && input[y + 1][x + 1] == b'A'
            && input[y + 2][x + 2] == b'M'
            && input[y + 3][x + 3] == b'X',
    ]
    .iter()
    .filter(|b| **b)
    .count()
}
