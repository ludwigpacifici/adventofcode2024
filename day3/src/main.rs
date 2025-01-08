fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.as_bytes();

    let part1 = parse_1(input);
    println!("part1: {part1}");
    assert_eq!(part1, 196826776);

    let part2 = parse_2(input);
    println!("part2: {part2}");
    assert_eq!(part2, 106780429);
}

fn parse_1(s: &[u8]) -> u64 {
    let mut at = 0;
    let mut score = 0;

    while at < s.len() {
        match parse_mul(s, at) {
            (Some(n), new_at) => {
                score += n;
                at = new_at;
            }
            // mul_parsing failed at the very beginning, therefore
            // nothing was consume. Manually look at the next
            // position.
            (None, new_at) if new_at == at => {
                at += 1;
            }
            // Some
            (None, new_at) => {
                at = new_at;
            }
        }
    }

    score
}

fn parse_2(s: &[u8]) -> u64 {
    let mut at = 0;
    let mut score = 0;
    let mut enable = true;

    while at < s.len() {
        if let (true, new_at) = parse_do(s, at) {
            enable = true;
            at = new_at;
        } else if let (true, new_at) = parse_dont(s, at) {
            enable = false;
            at = new_at;
        } else {
            match parse_mul(s, at) {
                (Some(n), new_at) => {
                    if enable {
                        score += n;
                    }
                    at = new_at;
                }
                // mul_parsing failed at the very beginning, therefore
                // nothing was consume. Manually look at the next
                // position.
                (None, new_at) if new_at == at => {
                    at += 1;
                }
                // Some
                (None, new_at) => {
                    at = new_at;
                }
            }
        }
    }

    score
}

fn parse_do(s: &[u8], at: usize) -> (bool, usize) {
    if s[at..].starts_with(b"do()") {
        let at = at + 4;
        (true, at)
    } else {
        (false, at)
    }
}

fn parse_dont(s: &[u8], at: usize) -> (bool, usize) {
    if s[at..].starts_with(b"don't") {
        let at = at + 7;
        (true, at)
    } else {
        (false, at)
    }
}

fn parse_mul(s: &[u8], at: usize) -> (Option<u64>, usize) {
    // try to match mul(
    if s[at..].starts_with(b"mul(") {
        let at = at + 4;
        if let (Some(lhs), at) = parse_up_to_3_digits_2(s, at) {
            if s[at] == b',' {
                let at = at + 1;
                if let (Some(rhs), at) = parse_up_to_3_digits_2(s, at) {
                    if s[at] == b')' {
                        let at = at + 1;
                        (Some(lhs * rhs), at)
                    } else {
                        (None, at)
                    }
                } else {
                    (None, at)
                }
            } else {
                (None, at)
            }
        } else {
            (None, at)
        }
    } else {
        // Could be more clever since it does not say what matched
        // from 'mul('.
        (None, at)
    }
}

fn parse_1_digit(s: &[u8], at: usize) -> (Option<u64>, usize) {
    if s[at].is_ascii_digit() {
        let n = (s[at] - b'0') as u64;
        let at = at + 1;
        (Some(n), at)
    } else {
        (None, at)
    }
}

fn parse_up_to_3_digits_2(s: &[u8], mut at: usize) -> (Option<u64>, usize) {
    let mut acc = None;
    for _ in 0..3 {
        match parse_1_digit(s, at) {
            (Some(n), new_at) => {
                acc = Some(10 * acc.unwrap_or_default() + n);
                at = new_at;
            }
            (None, new_at) => return (acc, new_at),
        }
    }

    (acc, at)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_1_tests() {
        const INPUT: &[u8] =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#.as_bytes();

        assert_eq!(parse_mul(&INPUT, 0), (None, 0));
        assert_eq!(parse_mul(&INPUT, 1), (Some(8), 9));
        assert_eq!(parse_mul(&INPUT, 29), (Some(25), 37));
        assert_eq!(parse_mul(&INPUT, 53), (Some(88), 62));
        assert_eq!(parse_mul(&INPUT, 62), (Some(40), 70));
    }

    #[test]
    fn parse_2_tests() {
        const INPUT: &[u8] =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#
                .as_bytes();

        assert_eq!(parse_do(&INPUT, 0), (false, 0));
        assert_eq!(parse_do(&INPUT, 59), (true, 63));

        assert_eq!(parse_dont(&INPUT, 0), (false, 0));
        assert_eq!(parse_dont(&INPUT, 20), (true, 27));
    }
}
