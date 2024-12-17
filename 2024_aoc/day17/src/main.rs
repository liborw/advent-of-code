use std::ops::BitXor;

use itertools::Itertools;
use utils::{took, run_task};
use rayon::prelude::*;

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    ins_p: usize,
    prog: Vec<u8>,
}

impl Computer {

    fn new(reg_a: usize, reg_b: usize, reg_c: usize, prog: Vec<u8>) -> Self {
        Computer{
            reg_a,
            reg_b,
            reg_c,
            ins_p: 0,
            prog
        }
    }

    fn get_combo_operad(&self, pointer: u8) -> usize {
        match pointer {
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            a if a <= 3 => a as usize,
            a => panic!("Not a valid program: {a}")
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut vec = vec![];
        while let Ok(v) = self.step() {
            if let Some(v) = v {
                vec.push(v);
            }
        }
        vec
    }

    fn step(&mut self) -> Result<Option<u8>, ()> {

        if self.ins_p + 1 >= self.prog.len() {
            return Err(());
        }

        let ins = self.prog[self.ins_p];
        let ope = self.prog[self.ins_p + 1];

        let v = match ins {
            // The adv instruction (opcode 0) performs division. The numerator is the value in
            // the A register. The denominator is found by raising 2 to the power of the
            // instruction's combo operand. (So, an operand of 2 would divide A by 4 (2^2);
            // an operand of 5 would divide A by 2^B.) The result of the division operation is
            // truncated to an integer and then written to the A register.
            0 => {
                self.reg_a = adv(self.reg_a,self.get_combo_operad(ope));
                self.ins_p += 2;
                None
            }
            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the
            // instruction's literal operand, then stores the result in register B.
            1 => {
                self.reg_b = self.reg_b.bitxor(ope as usize);
                self.ins_p += 2;
                None
            }
            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
            // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => {
                self.reg_b = self.get_combo_operad(ope) % 8;
                self.ins_p += 2;
                None
            }
            // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the
            // A register is not zero, it jumps by setting the instruction pointer to the value
            // of its literal operand; if this instruction jumps, the instruction pointer is not
            // increased by 2 after this instruction.
            3 => {
                if self.reg_a == 0 {
                    self.ins_p += 2;
                } else {
                    self.ins_p = ope as usize;
                }
                None
            }
            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C,
            // then stores the result in register B. (For legacy reasons, this instruction reads an
            // operand but ignores it.)
            4 => {
                self.reg_b = self.reg_b.bitxor(self.reg_c);
                self.ins_p += 2;
                None
            }
            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8,
            // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
            5 => {
                self.ins_p += 2;
                Some((self.get_combo_operad(ope) % 8) as u8)
            }
            // The bdv instruction (opcode 6) works exactly like the adv instruction except
            // that the result is stored in the B register. (The numerator is still read from the A register.)
            6 => {
                self.reg_b = adv(self.reg_a,self.get_combo_operad(ope));
                self.ins_p += 2;
                None
            }
            // The cdv instruction (opcode 7) works exactly like the adv instruction except that
            // the result is stored in the C register. (The numerator is still read from the A register.)
            7 => {
                self.reg_c = adv(self.reg_a,self.get_combo_operad(ope));
                self.ins_p += 2;
                None
            }
            v => panic!("Not valid instruction: {v}")
        };
        Ok(v)
    }
}

fn adv(a: usize, b:usize) -> usize {
    let denominator = 2_usize.pow(b as u32);
    if denominator == 0 {
        0
    } else {
        a / denominator
    }
}

fn parse(input: &str) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let (reg_a, reg_b, reg_c) = registers
        .lines()
        .map(|l| l[12..].parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();
    let prog = program[9..]
        .strip_suffix("\n")
        .unwrap()
        .split(",")
        .map(|v| v.parse::<u8>().unwrap())
        .collect();

    Computer::new(reg_a, reg_b, reg_c, prog)
}


fn part1(input: &str) -> String {
    let mut computer = parse(input);
    let vec = computer.run();
    vec.into_iter().map(|v| format!("{}", v)).join(",")
}

fn part2(input: &str) -> usize {
    let computer = parse(input);

    (1..1000000000000).into_par_iter().find_any(|a| {
        let mut c = computer.clone();
        c.reg_a = *a;
        computer.prog == c.run()
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_mini1_test() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        let out = computer.run();
        assert_eq!(out, vec![]);
        assert_eq!(computer.reg_b, 1);
    }

    #[test]
    fn day17_mini2_test() {
        let mut computer = Computer::new(10, 0, 0, vec![5,0,5,1,5,4]);
        let out = computer.run();
        assert_eq!(out, vec![0,1,2]);
    }

    #[test]
    fn day17_mini3_test() {
        let mut computer = Computer::new(2024, 0, 0, vec![0,1,5,4,3,0]);
        let out = computer.run();
        assert_eq!(out, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.reg_a, 0);
    }

    #[test]
    fn day17_mini4_test() {
        let mut computer = Computer::new(0, 29, 0, vec![1,7]);
        let out = computer.run();
        assert_eq!(out, vec![]);
        assert_eq!(computer.reg_b, 26);
    }

    #[test]
    fn day17_mini5_test() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4,0]);
        let out = computer.run();
        assert_eq!(out, vec![]);
        assert_eq!(computer.reg_b, 44354);
    }

    #[test]
    fn day17_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn day17_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), "");
    }

    #[test]
    fn day17_part2_test() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn day17_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
