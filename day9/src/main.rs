fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.trim().as_bytes();
    let capacity = input.iter().map(|b| (to_digit(*b)) as usize).sum::<usize>();
    let disk = make_disk(capacity, input);

    let part1 = checksum(&compact_1(disk.clone()));
    println!("part1: {part1}");
    assert_eq!(part1, 6283170117911);

    let part2 = checksum(&compact_2(input, disk.clone()));
    println!("part2: {part2}");
    assert_eq!(part2, 6307653242596);
}

fn compact_1(mut disk: Vec<u16>) -> Vec<u16> {
    let mut l = 0;
    let mut r = disk.len() - 1;
    while l < r {
        while disk[l] != u16::MAX && l < r {
            l += 1;
        }

        while disk[r] == u16::MAX && l < r {
            r -= 1;
        }

        disk.swap(l, r);
    }
    disk
}

fn compact_2(input: &[u8], mut disk: Vec<u16>) -> Vec<u16> {
    for i_input in (0..input.len()).step_by(2).rev() {
        let len = to_digit(input[i_input]) as usize;
        let id = (i_input / 2) as u16;

        if let Some(fill_at) = disk
            .windows(len)
            .enumerate()
            .take_while(|(i, _w)| disk[*i] != id)
            .find_map(|(i, w)| w.iter().all(|&b| b == u16::MAX).then_some(i))
        {
            let free_at = disk.iter().position(|other_id| *other_id == id).unwrap();
            disk[fill_at..fill_at + len].fill(id);
            disk[free_at..free_at + len].fill(u16::MAX);
        }
    }
    disk
}

fn checksum(disk: &[u16]) -> usize {
    disk.iter()
        .enumerate()
        .filter(|(_, n)| **n != u16::MAX)
        .map(|(i, b)| i * (*b as usize))
        .sum::<usize>()
}

fn make_disk(capacity: usize, input: &[u8]) -> Vec<u16> {
    let mut disk = Vec::with_capacity(capacity);
    let mut id = 0u16;
    let mut is_file = true;

    for &n in input {
        let n = to_digit(n);
        let fill_with = if is_file {
            id += 1;
            id - 1
        } else {
            u16::MAX
        };
        for _ in 0..n {
            disk.push(fill_with);
        }
        is_file = !is_file;
    }

    disk
}

fn to_digit(n: u8) -> u8 {
    n - b'0'
}
