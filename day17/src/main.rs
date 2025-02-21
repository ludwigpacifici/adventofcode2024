fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (registers, code) = parse(&input);
    let p = Program::new(registers, code);

    let mut part1 = algo(60589763)
        .into_iter()
        .fold(String::new(), |mut acc, n| {
            acc.push_str(&n.to_string());
            acc.push(',');
            acc
        });
    part1.pop();
    println!("part1: {part1}");
    assert_eq!(part1, "3,5,0,1,5,1,5,1,0");

    let part2 = part_2(&p.code, p.code.len() as isize - 1, 0).unwrap();
    println!("part2: {part2}");
    assert_eq!(part2, 107413700225434);
}

fn part_2(code: &[Integer], p: isize, a: Integer) -> Option<Integer> {
    if p < 0 {
        return Some(a);
    }

    for i in 0..8 {
        let out = algo((a << 3) | i);
        if !out.is_empty() && out[0] == code[p as usize] {
            if let Some(a) = part_2(code, p - 1, a << 3 | i) {
                return Some(a);
            }
        }
    }

    None
}

fn algo(mut a: Integer) -> Vec<Integer> {
    let mut out = Vec::new();
    while a > 0 {
        // Input code 'decompiled':
        // let mut b = a & 0b111;
        // b = b ^ 0b101;
        // let c = a >> b;
        // b = b ^ 0b110;
        // b = b ^ c;
        // out.push(b & 0b111);
        // a = a >> 3;
        // Then shorten to:
        out.push(((a & 0b111) ^ 0b011 ^ (a >> ((a & 0b111) ^ 0b101))) & 0b111);
        a >>= 3;
    }
    out
}

type Integer = u64;

#[derive(Debug)]
struct Program {
    registers: [Integer; 3],
    out: Vec<Integer>,
    ip: usize,
    code: Vec<Integer>,
}

fn parse(input: &str) -> ([Integer; 3], Vec<Integer>) {
    let it = input.lines();
    let (it, a) = parse_register(it);
    let (it, b) = parse_register(it);
    let (it, c) = parse_register(it);
    let p = parse_program(it);
    ([a, b, c], p)
}

fn parse_register<'a>(
    mut it: impl Iterator<Item = &'a str>,
) -> (impl Iterator<Item = &'a str>, Integer) {
    let v = it
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<Integer>()
        .unwrap();
    (it, v)
}

fn parse_program<'a>(mut it: impl Iterator<Item = &'a str>) -> Vec<Integer> {
    it.nth(1)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|n| n.parse::<Integer>().unwrap())
        .collect::<Vec<_>>()
}

// Below code writen to run the VM. Later, shorten into function called 'algo'

// #[derive(Debug)]
// enum Combo {
//     Integer(Integer),
//     RegisterName(Register),
//     Reserved,
// }

// impl From<Integer> for Combo {
//     fn from(value: Integer) -> Self {
//         match value {
//             0..=3 => Combo::Integer(value),
//             4 => Combo::RegisterName(Register::A),
//             5 => Combo::RegisterName(Register::B),
//             6 => Combo::RegisterName(Register::C),
//             7 => Combo::Reserved,
//             n => panic!("Unknown combo operand: {n}"),
//         }
//     }
// }

// #[derive(Debug)]
// enum Register {
//     A,
//     B,
//     C,
// }

// impl Index<Register> for [Integer; 3] {
//     type Output = Integer;

//     fn index(&self, index: Register) -> &Self::Output {
//         match index {
//             Register::A => &self[0],
//             Register::B => &self[1],
//             Register::C => &self[2],
//         }
//     }
// }

// impl IndexMut<Register> for [Integer; 3] {
//     fn index_mut(&mut self, index: Register) -> &mut Self::Output {
//         match index {
//             Register::A => &mut self[0],
//             Register::B => &mut self[1],
//             Register::C => &mut self[2],
//         }
//     }
// }

impl Program {
    fn new(registers: [Integer; 3], code: Vec<Integer>) -> Self {
        Self {
            registers,
            out: Vec::with_capacity(code.len()),
            ip: 0,
            code,
        }
    }

    //     fn run_1(&mut self) {
    //         loop {
    //             self.run_one();
    //         }
    //     }

    //     fn run_one(&mut self) {
    //         self.check_halt();
    //         let opcode = self.code[self.ip];
    //         match opcode {
    //             0 => self.adv(),
    //             1 => self.bxl(),
    //             2 => self.bst(),
    //             3 => self.jnz(),
    //             4 => self.bxc(),
    //             5 => self.out(),
    //             6 => self.bdv(),
    //             7 => self.cdv(),
    //             _ => panic!("Unknown opcode {opcode}"),
    //         }
    //     }

    //     fn adv(&mut self) {
    //         let d = self.combo(self.read_combo());
    //         *self.a_mut() = self.a() >> d;
    //         self.ip += 2;
    //     }

    //     fn bxl(&mut self) {
    //         *self.b_mut() = self.b() ^ self.read_literal();
    //         self.ip += 2;
    //     }

    //     fn bst(&mut self) {
    //         *self.b_mut() = self.combo(self.read_combo()) & 0b111;
    //         self.ip += 2;
    //     }

    //     fn jnz(&mut self) {
    //         if self.a() == 0 {
    //             self.ip += 2;
    //         } else {
    //             self.ip = self.read_literal() as usize;
    //         }
    //     }

    //     fn bxc(&mut self) {
    //         *self.b_mut() = self.b() ^ self.c();
    //         self.ip += 2;
    //     }

    //     fn out(&mut self) {
    //         let c = self.combo(self.read_combo()) & 0b111;
    //         self.out.push(c);
    //         // print!("{},", c);
    //         self.ip += 2;
    //     }

    //     fn bdv(&mut self) {
    //         let d = self.combo(self.read_combo());
    //         *self.b_mut() = self.a() >> d;
    //         self.ip += 2;
    //     }

    //     fn cdv(&mut self) {
    //         let d = self.combo(self.read_combo());
    //         *self.c_mut() = self.a() >> d;
    //         self.ip += 2;
    //     }

    //     fn read_literal(&self) -> Integer {
    //         self.code[self.ip + 1]
    //     }

    //     fn read_combo(&self) -> Combo {
    //         Combo::from(self.code[self.ip + 1])
    //     }

    //     fn check_halt(&self) {
    //         if self.code.len() <= self.ip {
    //             panic!("HALT");
    //         }
    //     }

    //     // fn is_halt(&self) -> bool {
    //     //     self.code.len() <= self.ip
    //     // }

    //     fn combo(&self, c: Combo) -> Integer {
    //         match c {
    //             Combo::Integer(n) => n,
    //             Combo::RegisterName(r) => self.registers[r],
    //             Combo::Reserved => panic!("accessing Reserved combo"),
    //         }
    //     }

    //     fn a(&self) -> Integer {
    //         self.registers[Register::A]
    //     }
    //     fn b(&self) -> Integer {
    //         self.registers[Register::B]
    //     }
    //     fn c(&self) -> Integer {
    //         self.registers[Register::C]
    //     }

    //     fn a_mut(&mut self) -> &mut Integer {
    //         &mut self.registers[Register::A]
    //     }
    //     fn b_mut(&mut self) -> &mut Integer {
    //         &mut self.registers[Register::B]
    //     }
    //     fn c_mut(&mut self) -> &mut Integer {
    //         &mut self.registers[Register::C]
    //     }
}
