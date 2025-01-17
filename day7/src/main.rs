fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let equations = input.lines().map(|l| {
        let (l, r) = l.split_once(": ").unwrap();
        let target = l.parse::<u64>().unwrap();
        let numbers = r
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        (target, numbers)
    });

    let part1 = part1(equations.clone());
    println!("part1: {part1}");
    assert_eq!(part1, 3245122495150);

    let part2 = part2(equations);
    println!("part2: {part2}");
    assert_eq!(part2, 105517128211543);
}

fn part1(equations: impl Iterator<Item = (u64, Vec<u64>)>) -> u64 {
    equations
        .filter_map(|(target, ns)| is_solution_1(target, &ns).then_some(target))
        .sum()
}

fn is_solution_1(target: u64, ns: &[u64]) -> bool {
    let op_len = ns.len() - 1;
    for choices in 0..(1 << op_len) {
        // invariant at least two numbers are available.
        debug_assert!(ns.len() > 1);

        let mut solution = ns[0];

        for i in 0..op_len {
            let is_addition = (choices >> i) & 1 == 0;
            if is_addition {
                solution += ns[i + 1];
            } else {
                solution *= ns[i + 1];
            }
            if solution > target {
                break;
            }
        }

        if solution == target {
            return true;
        }
    }
    false
}

fn part2(equations: impl Iterator<Item = (u64, Vec<u64>)>) -> u64 {
    equations
        .filter_map(|(target, ns)| is_solution_2(target, &ns, 0).then_some(target))
        .sum()
}

fn is_solution_2(target: u64, ns: &[u64], acc: u64) -> bool {
    if acc > target {
        return false;
    }

    match ns {
        [] => acc == target,
        [n, rest @ ..] => {
            is_solution_2(target, rest, acc + n)
                || is_solution_2(target, rest, acc * n)
                || is_solution_2(target, rest, concat(acc, *n))
        }
    }
}

fn concat(mut a: u64, b: u64) -> u64 {
    a *= 10_u64.pow(b.ilog10() + 1);
    a + b
}
