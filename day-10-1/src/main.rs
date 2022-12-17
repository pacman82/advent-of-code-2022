//! --- Day 10: Cathode-Ray Tube ---
//!
//! You avoid the ropes, plunge into the river, and swim to shore.
//! The Elves yell something about meeting back up with them upriver, but the river is too loud to
//! tell exactly what they're saying. They finish crossing the bridge and disappear from view.
//! Situations like this must be why the Elves prioritized getting the communication system on your
//! handheld device working. You pull it out of your pack, but the amount of water slowly draining
//! from a big crack in its screen tells you it probably won't be of much immediate use.
//! Unless, that is, you can design a replacement for the device's video system! It seems to be some
//! kind of cathode-ray tube screen and simple CPU that are both driven by a precise clock circuit.
//! The clock circuit ticks at a constant rate; each tick is called a cycle.
//!
//! Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which
//! starts with the value 1. It supports only two instructions:
//!
//!     addx V takes two cycles to complete. After two cycles, the X register is increased by the
//! value V. (V can be negative.)     noop takes one cycle to complete. It has no other effect.
//!
//! The CPU uses these instructions in a program (your puzzle input) to, somehow, tell the screen
//! what to draw. Consider the following small program:
//!
//! ```
//! noop
//! addx 3
//! addx -5
//! ```
//!
//! Execution of this program proceeds as follows:
//!
//! * At the start of the first cycle, the noop instruction begins execution. During the first
//!   cycle, X is 1. After the first cycle, the noop instruction finishes execution, doing nothing.
//! * At the start of the second cycle, the addx 3 instruction begins execution. During the second
//!   cycle, X is still 1.
//! * During the third cycle, X is still 1. After the third cycle, the addx 3 instruction finishes
//!   execution, setting X to 4.
//! * At the start of the fourth cycle, the addx -5 instruction begins execution. During the fourth
//!   cycle, X is still 4.
//! * During the fifth cycle, X is still 4. After the fifth cycle, the addx -5 instruction finishes
//!   execution, setting X to -1.
//!
//! Maybe you can learn something by looking at the value of the X register throughout execution.
//! For now, consider the signal strength (the cycle number multiplied by the value of the X
//! register) during the 20th cycle and every 40 cycles after that (that is, during the 20th, 60th,
//! 100th, 140th, 180th, and 220th cycles).
//!
//! For example, consider this larger program:
//!
//! ```
//! addx 15
//! addx -11
//! addx 6
//! addx -3
//! addx 5
//! addx -1
//! addx -8
//! addx 13
//! addx 4
//! noop
//! addx -1
//! addx 5
//! addx -1
//! addx 5
//! addx -1
//! addx 5
//! addx -1
//! addx 5
//! addx -1
//! addx -35
//! addx 1
//! addx 24
//! addx -19
//! addx 1
//! addx 16
//! addx -11
//! noop
//! noop
//! addx 21
//! addx -15
//! noop
//! noop
//! addx -3
//! addx 9
//! addx 1
//! addx -3
//! addx 8
//! addx 1
//! addx 5
//! noop
//! noop
//! noop
//! noop
//! noop
//! addx -36
//! noop
//! addx 1
//! addx 7
//! noop
//! noop
//! noop
//! addx 2
//! addx 6
//! noop
//! noop
//! noop
//! noop
//! noop
//! addx 1
//! noop
//! noop
//! addx 7
//! addx 1
//! noop
//! addx -13
//! addx 13
//! addx 7
//! noop
//! addx 1
//! addx -33
//! noop
//! noop
//! noop
//! addx 2
//! noop
//! noop
//! noop
//! addx 8
//! noop
//! addx -1
//! addx 2
//! addx 1
//! noop
//! addx 17
//! addx -9
//! addx 1
//! addx 1
//! addx -3
//! addx 11
//! noop
//! noop
//! addx 1
//! noop
//! addx 1
//! noop
//! noop
//! addx -13
//! addx -19
//! addx 1
//! addx 3
//! addx 26
//! addx -30
//! addx 12
//! addx -1
//! addx 3
//! addx 1
//! noop
//! noop
//! noop
//! addx -9
//! addx 18
//! addx 1
//! addx 2
//! noop
//! noop
//! addx 9
//! noop
//! noop
//! noop
//! addx -1
//! addx 2
//! addx -37
//! addx 1
//! addx 3
//! noop
//! addx 15
//! addx -21
//! addx 22
//! addx -6
//! addx 1
//! noop
//! addx 2
//! addx 1
//! noop
//! addx -10
//! noop
//! noop
//! addx 20
//! addx 1
//! addx 2
//! addx 2
//! addx -6
//! addx -11
//! noop
//! noop
//! noop
//! ```
//!
//! The interesting signal strengths can be determined as follows:
//!
//! ```
//! During the 20th cycle, register X has the value 21, so the signal strength is 20 * 21 = 420. (The 20th cycle occurs in the middle of the second addx -1, so the value of register X is the starting value, 1, plus all of the other addx values up to that point: 1 + 15 - 11 + 6 - 3 + 5 - 1 - 8 + 13 + 4 = 21.)
//! During the 60th cycle, register X has the value 19, so the signal strength is 60 * 19 = 1140.
//! During the 100th cycle, register X has the value 18, so the signal strength is 100 * 18 = 1800.
//! During the 140th cycle, register X has the value 21, so the signal strength is 140 * 21 = 2940.
//! During the 180th cycle, register X has the value 16, so the signal strength is 180 * 16 = 2880.
//! During the 220th cycle, register X has the value 18, so the signal strength is 220 * 18 = 3960.
//! ```
//!
//! The sum of these signal strengths is 13140.
//!
//! Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles. **What is
//! the sum of these six signal strengths?**

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use atoi::FromRadix10SignedChecked;

use lines::InputIterator;

mod lines;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("Can not open input file"));
    let n = sum_signal_strength(input);
    println!("{n}");
}

fn sum_signal_strength(input: impl BufRead) -> i64 {
    let interesting_cycles = [20, 60, 100, 140, 180, 220];
    let instructions = InputIterator::new(input, Instruction::from_line);
    let mut cc = ClockCircuit::new(instructions);
    let mut sum_signal_strength = 0;
    for cycle in 1..=220 {
        if interesting_cycles.contains(&cycle) {
            sum_signal_strength += cc.signal_strength();
        }
        cc.tick();
    }
    sum_signal_strength
}

struct ClockCircuit<I> {
    instructions: I,
    register: Option<i64>,
    x: i64,
    cycle: i64,
}

impl<I> ClockCircuit<I> {
    fn new(instructions: I) -> Self {
        Self {
            x: 1,
            cycle: 1,
            register: None,
            instructions,
        }
    }

    fn tick(&mut self)
    where
        I: Iterator<Item = Instruction>,
    {
        if let Some(add_me) = self.register {
            self.x += add_me;
            self.register = None;
        } else {
            let inst = self.instructions.next().expect("yagni");
            match inst {
                Instruction::Noop => (),
                Instruction::AddX(amount) => {
                    self.register = Some(amount);
                }
            }
        }
        self.cycle += 1;
    }

    fn x(&self) -> i64 {
        self.x
    }

    fn signal_strength(&self) -> i64 {
        self.x() * self.cycle
    }
}

enum Instruction {
    Noop,
    AddX(i64),
}

impl Instruction {
    fn from_line(line: &[u8]) -> Self {
        match &line[..4] {
            b"noop" => Self::Noop,
            b"addx" => {
                let (amount, _) = i64::from_radix_10_signed_checked(&line[5..]);
                Self::AddX(amount.expect("Invalid Addx syntax"))
            }
            _ => panic!("yagni"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lines::InputIterator, sum_signal_strength, ClockCircuit, Instruction};

    #[test]
    fn small_program() {
        let input = "\
            noop\n\
            addx 3\n\
            addx -5\n\
        ";
        let instructions = InputIterator::new(input.as_bytes(), Instruction::from_line);
        let mut cc = ClockCircuit::new(instructions);

        assert_eq!(1, cc.x());
        cc.tick();
        assert_eq!(1, cc.x());
        cc.tick();
        assert_eq!(1, cc.x());
        cc.tick();
        assert_eq!(4, cc.x());
        cc.tick();
        assert_eq!(4, cc.x());
        cc.tick();
        assert_eq!(-1, cc.x());
    }

    #[test]
    fn larger_programm() {
        let input = "\
            addx 15\n\
            addx -11\n\
            addx 6\n\
            addx -3\n\
            addx 5\n\
            addx -1\n\
            addx -8\n\
            addx 13\n\
            addx 4\n\
            noop\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx 5\n\
            addx -1\n\
            addx -35\n\
            addx 1\n\
            addx 24\n\
            addx -19\n\
            addx 1\n\
            addx 16\n\
            addx -11\n\
            noop\n\
            noop\n\
            addx 21\n\
            addx -15\n\
            noop\n\
            noop\n\
            addx -3\n\
            addx 9\n\
            addx 1\n\
            addx -3\n\
            addx 8\n\
            addx 1\n\
            addx 5\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            addx -36\n\
            noop\n\
            addx 1\n\
            addx 7\n\
            noop\n\
            noop\n\
            noop\n\
            addx 2\n\
            addx 6\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            noop\n\
            addx 1\n\
            noop\n\
            noop\n\
            addx 7\n\
            addx 1\n\
            noop\n\
            addx -13\n\
            addx 13\n\
            addx 7\n\
            noop\n\
            addx 1\n\
            addx -33\n\
            noop\n\
            noop\n\
            noop\n\
            addx 2\n\
            noop\n\
            noop\n\
            noop\n\
            addx 8\n\
            noop\n\
            addx -1\n\
            addx 2\n\
            addx 1\n\
            noop\n\
            addx 17\n\
            addx -9\n\
            addx 1\n\
            addx 1\n\
            addx -3\n\
            addx 11\n\
            noop\n\
            noop\n\
            addx 1\n\
            noop\n\
            addx 1\n\
            noop\n\
            noop\n\
            addx -13\n\
            addx -19\n\
            addx 1\n\
            addx 3\n\
            addx 26\n\
            addx -30\n\
            addx 12\n\
            addx -1\n\
            addx 3\n\
            addx 1\n\
            noop\n\
            noop\n\
            noop\n\
            addx -9\n\
            addx 18\n\
            addx 1\n\
            addx 2\n\
            noop\n\
            noop\n\
            addx 9\n\
            noop\n\
            noop\n\
            noop\n\
            addx -1\n\
            addx 2\n\
            addx -37\n\
            addx 1\n\
            addx 3\n\
            noop\n\
            addx 15\n\
            addx -21\n\
            addx 22\n\
            addx -6\n\
            addx 1\n\
            noop\n\
            addx 2\n\
            addx 1\n\
            noop\n\
            addx -10\n\
            noop\n\
            noop\n\
            addx 20\n\
            addx 1\n\
            addx 2\n\
            addx 2\n\
            addx -6\n\
            addx -11\n\
            noop\n\
            noop\n\
            noop\n\
        ";

        let sum = sum_signal_strength(input.as_bytes());

        assert_eq!(13140, sum);
    }
}
