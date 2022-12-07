//! --- Day 5: Supply Stacks ---
//!
//! The expedition can depart as soon as the final supplies have been unloaded from the ships.
//! Supplies are stored in stacks of marked crates, but because the needed supplies are buried under
//! many other crates, the crates need to be rearranged. The ship has a giant cargo crane capable of
//! moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane
//! operator will rearrange them in a series of carefully-planned steps. After the crates are
//! rearranged, the desired crates will be at the top of each stack. The Elves don't want to
//! interrupt the crane operator during this delicate procedure, but they forgot to ask her which
//! crate will end up where, and they want to be ready to unload them as soon as possible so they
//! can embark. They do, however, have a drawing of the starting stacks of crates and the
//! rearrangement procedure (your puzzle input). For example:
//!
//! ```
//!     [D]    
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! ```
//!
//! In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on
//! the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are
//! crates M, C, and D. Finally, stack 3 contains a single crate, P. Then, the rearrangement
//! procedure is given. In each step of the procedure, a quantity of crates is moved from one stack
//! to a different stack. In the first step of the above rearrangement procedure, one crate is moved
//! from stack 2 to stack 1, resulting in this configuration:
//!
//! ```
//! [D]        
//! [N] [C]    
//! [Z] [M] [P]
//!  1   2   3
//! ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a
//! time, so the first crate to be moved (D) ends up below the second and third crates:
//!
//! ```
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//! ```
//!
//! Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a
//! time, crate C ends up below crate M:
//!
//! ```
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//! ```
//!
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//! ```
//!
//! The Elves just need to know which crate will end up on top of each stack; in this example, the
//! top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these
//! together and give the Elves the message CMZ.
//!
//! **After the rearrangement procedure completes, what crate ends up on top of each stack?**

use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Error;
use atoi::FromRadix10Checked;
use lines::LineStream;

mod lines;

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input.txt")?);
    let message = top_crates(input);
    println!("{message}");
    Ok(())
}

fn top_crates(input: impl BufRead) -> String {
    let mut lines = LineStream::new(input);
    let mut crates = Crates::from_lines(&mut lines);
    lines.next(); // Jump over empty line

    while let Some(line) = lines.next() {
        let instruction = Instruction::from_line(line);
        crates.apply(&instruction);
        eprintln!("{}", crates.top_crates());
    }

    crates.top_crates()
}

struct Crates {
    buffer: Vec<char>,
    // Stacks with crates in bottom up order (the lowest crate is first).
    stacks: Vec<Vec<char>>,
}

impl Crates {
    fn from_lines(lines: &mut LineStream<impl BufRead>) -> Self {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        while let Some(line) = lines.next() {
            if !line.contains(&b'[') {
                break;
            }
            for (stack_index, byte_index) in (1..line.len()).step_by(4).enumerate() {
                match line[byte_index] {
                    b' ' => (),
                    item => {
                        stacks.resize_with(max(stacks.len(), stack_index + 1), Vec::new);
                        stacks[stack_index].push(item as char)
                    }
                }
            }
        }

        for stack in &mut stacks {
            stack.reverse();
        }

        Self {
            buffer: Vec::new(),
            stacks,
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        self.buffer.clear();
        let from = &mut self.stacks[instruction.from];
        let at = from.len() - instruction.amount;
        self.buffer.extend_from_slice(&from[at..]);
        self.buffer.reverse();
        from.resize(at, '\0');
        let to = &mut self.stacks[instruction.to];
        to.extend_from_slice(&self.buffer);
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().copied().unwrap_or_default())
            .collect()
    }
}

struct Instruction {
    /// Number of crates to move
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_line(line: &[u8]) -> Instruction {
        let (amount, digits_amount) = usize::from_radix_10_checked(&line[5..]);
        let (from, digits_from) = usize::from_radix_10_checked(&line[(5 + 6 + digits_amount)..]);
        let (to, _) =
            usize::from_radix_10_checked(&line[(5 + 6 + 4 + digits_amount + digits_from)..]);
        Instruction {
            amount: amount.unwrap(),
            from: from.unwrap() - 1,
            to: to.unwrap() - 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::top_crates;


    #[test]
    fn top_crates_example_given (){
        let input = "    \
                [D]           \n\
            [N] [C]           \n\
            [Z] [M] [P]       \n \
             1   2   3        \n\
            \n\
            move 1 from 2 to 1\n\
            move 3 from 1 to 3\n\
            move 2 from 2 to 1\n\
            move 1 from 1 to 2\n\
        ";

        let actual = top_crates(Cursor::new(input));

        assert_eq!("CMZ", actual);
    }
}