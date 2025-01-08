use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let input = input.lines().map(|l| {
        let mut it = l.split_ascii_whitespace();
        let fst = it.next().unwrap().parse::<u32>().unwrap();
        let snd = it.next().unwrap().parse::<u32>().unwrap();
        (fst, snd)
    });

    let (mut fsts, mut snds) = input.clone().fold(
        (Vec::new(), Vec::new()),
        |(mut fsts, mut snds), (fst, snd)| {
            fsts.push(fst);
            snds.push(snd);
            (fsts, snds)
        },
    );

    fsts.sort_unstable();
    snds.sort_unstable();

    let part1 = fsts
        .into_iter()
        .zip(snds)
        .map(|(f, s)| f.abs_diff(s))
        .sum::<u32>();

    println!("part1: {part1}");

    let (fsts, snds) = input.fold(
        (HashMap::<u32, usize>::new(), HashMap::<u32, usize>::new()),
        |(mut fsts, mut snds), (fst, snd)| {
            *fsts.entry(fst).or_default() += 1;
            *snds.entry(snd).or_default() += 1;
            (fsts, snds)
        },
    );

    let part2 = fsts
        .into_iter()
        .map(|(f, count)| (f as usize) * snds.get(&f).cloned().unwrap_or_default() * count)
        .sum::<usize>();
    println!("part2: {part2}");

    assert_eq!(part1, 2285373);
    assert_eq!(part2, 21142653);
}
