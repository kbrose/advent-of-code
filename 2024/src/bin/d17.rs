use std::fs;

#[derive(Debug, Clone)]
struct Interpreter {
    program: Vec<u8>,
    instruction_pointer: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

enum StepResult {
    Out(u8),
    Halt,
    Continue,
}

impl Interpreter {
    fn step(&mut self) -> StepResult {
        let opcode = self.program.get(self.instruction_pointer);
        let operand = self.program.get(self.instruction_pointer + 1);
        match (opcode, operand) {
            // 0: adv
            (Some(0), Some(operand)) => {
                self.reg_a /= 2_u64.pow(self.combo(*operand) as u32);
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            // 1: bxl
            (Some(1), Some(operand)) => {
                self.reg_b ^= *operand as u64;
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            // 2: bst
            (Some(2), Some(operand)) => {
                self.reg_b = self.combo(*operand) % 8;
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            // 3: jnz
            (Some(3), Some(operand)) => {
                if self.reg_a == 0 {
                    self.instruction_pointer += 2;
                } else {
                    self.instruction_pointer = *operand as usize;
                }
                StepResult::Continue
            }
            // 4: bxc
            (Some(4), Some(_)) => {
                self.reg_b ^= self.reg_c;
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            // 5: out
            (Some(5), Some(operand)) => {
                self.instruction_pointer += 2;
                StepResult::Out((self.combo(*operand) % 8) as u8)
            }
            // 6: bdv
            (Some(6), Some(operand)) => {
                self.reg_b = self.reg_a / 2_u64.pow(self.combo(*operand) as u32);
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            // 7: cdv
            (Some(7), Some(operand)) => {
                self.reg_c = self.reg_a / 2_u64.pow(self.combo(*operand) as u32);
                self.instruction_pointer += 2;
                StepResult::Continue
            }
            (None, _) => StepResult::Halt,
            _ => panic!("Unexpected state opcode {opcode:?} and operand {operand:?}"),
        }
    }

    fn combo(&self, n: u8) -> u64 {
        match n {
            0..=3 => n as u64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Unexpected combo operand {n}"),
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        loop {
            let result = self.step();
            match result {
                StepResult::Out(n) => {
                    out.push(n);
                }
                StepResult::Halt => {
                    break;
                }
                StepResult::Continue => {}
            }
        }
        out
    }
}

fn parse_input(contents: &str) -> Interpreter {
    let (reg_a_str, rest) = contents.trim().split_once('\n').unwrap();
    let (reg_b_str, rest) = rest.split_once('\n').unwrap();
    let (reg_c_str, program_str) = rest.split_once("\n\n").unwrap();

    let reg_a = reg_a_str.split(": ").nth(1).unwrap().parse().unwrap();
    let reg_b = reg_b_str.split(": ").nth(1).unwrap().parse().unwrap();
    let reg_c = reg_c_str.split(": ").nth(1).unwrap().parse().unwrap();

    let instruction_list = program_str
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Interpreter {
        reg_a,
        reg_b,
        reg_c,
        program: instruction_list,
        instruction_pointer: 0,
    }
}

fn compute_1(contents: &str) -> Vec<u8> {
    let mut interpreter = parse_input(contents);
    interpreter.run()
}

// To be honest, I cannot fully explain this one.
// After hand-inspecting the program, it became clear that it operated on 3 bits of A
// at a time: e.g. 0b_xxx_yyy_zzz will output three values, each one (mostly) driven by
// zzz, yyy, and xxx respectively.
//
// If the sets of 3 bits were totally independent, then this problem would be
// very simple. E.g. if for 0b_xxx_yyy_zzz, the first output value _only_ depended
// on the zzz bits, then it would be straight forward.
//
// However, it also became clear that consecutive groups of 3 bits _can_
// influence the output of each other, mostly because of the division instructions
// which are basically A >> n for some n=0..8 (n possibly being another register).
//
// To get around this, we keep track of all possible values of bits would result
// in the desired output. E.g. which 0b_zzz bits result in desired output value z.
// This is the part that I cannot explain why it works, the approach came to me when
// I was playing around with different inputs manually.
fn compute_2(contents: &str) -> u64 {
    let init_interpreter = parse_input(contents);
    let desired_outputs = init_interpreter.program.clone();
    let mut nums: Vec<u64> = vec![0];
    loop {
        let mut curr_nums: Vec<u64> = vec![];
        for mut num in nums.into_iter() {
            num <<= 3;
            for i in 0..8 {
                let mut interpreter = init_interpreter.clone();
                interpreter.reg_a = num + i;
                let outs = interpreter.run();
                if outs.len() > desired_outputs.len() {
                    panic!("Something has gone terribly wrong!");
                }
                if outs == desired_outputs {
                    return num + i;
                }
                if (!outs.is_empty())
                    && desired_outputs[(desired_outputs.len() - outs.len())..] == outs
                {
                    curr_nums.push(num + i);
                }
            }
        }
        nums = curr_nums;
    }
}

// "Compiled" program used for reasoning about what is happening.
#[allow(dead_code)]
fn program(mut a: u64) -> Vec<u64> {
    let mut out = vec![];
    loop {
        // bst
        let mut b = a & 0b_111;

        // bxl
        b ^= 0b_010;

        // // cdv (with `& 0b_111` added)
        // let c = (a >> b) & 0b_111;
        //
        // // bxc
        // b ^= c;

        // cdv + bxc
        b ^= (a >> b) & 0b_111;
        // bxl
        b ^= 0b_011;
        // out
        out.push(b & 0b_111);
        // adv
        a >>= 3;
        // jnz
        if a == 0 {
            break;
        }
    }
    out
}

fn main() {
    let contents =
        fs::read_to_string("inputs/d17.txt").expect("Should have been able to read the file");

    let result = compute_1(&contents);
    assert_eq!(vec![2, 7, 4, 7, 2, 1, 7, 5, 1], result);
    println!(
        "part 1: {}",
        result
            .into_iter()
            .map(|i| format!("{i}"))
            .collect::<Vec<String>>()
            .join(",")
    );

    let result = compute_2(&contents);
    assert_eq!(37221274271220, result);
    println!("part 2: {result}");
}
