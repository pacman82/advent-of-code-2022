//! --- Day 3: Rucksack Reorganization ---
//!
//! One Elf has the important job of loading all of the rucksacks with supplies for the jungle
//! journey. Unfortunately, that Elf didn't quite follow the packing instructions, and so a few
//! items now need to be rearranged. Each rucksack has two large compartments. All items of a given
//! type are meant to go into exactly one of the two compartments. The Elf that did the packing
//! failed to follow this rule for exactly one item type per rucksack. The Elves have made a list of
//! all of the items currently in each rucksack (your puzzle input), but they need your help finding
//! the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a
//! and A refer to different types of items). The list of items for each rucksack is given as
//! characters all on a single line. A given rucksack always has the same number of items in each of
//! its two compartments, so the first half of the characters represent items in the first
//! compartment, while the second half of the characters represent items in the second compartment.
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! ```
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//!     The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first
//! compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items
//! hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
//!     The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only
//! item type that appears in both compartments is uppercase L.     The third rucksack's
//! compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
//!     The fourth rucksack's compartments only share item type v.
//!     The fifth rucksack's compartments only share item type t.
//!     The sixth rucksack's compartments only share item type s.
//!
//! To help prioritize item rearrangement, every item type can be converted to a priority:
//!
//!     Lowercase item types a through z have priorities 1 through 26.
//!     Uppercase item types A through Z have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both compartments of each
//! rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
//! Find the item type that appears in both compartments of each rucksack. What is the sum of the
//! priorities of those item types?

use anyhow::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input.txt")?);
    let sum = sum_of_priorities(input);
    println!("{sum}");
    Ok(())
}

fn sum_of_priorities(input: impl BufRead) -> u64 {
    let mut sum = 0;
    let mut lines = LineStream::new(input);
    while let Some(rucksack) = lines.next() {
        sum += priority(common(rucksack)) as u64;
    }
    sum
}

const fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => 0,
    }
}

/// Item both compartments have in common
fn common(rucksack: &[u8]) -> u8 {
    let mid = rucksack.len() / 2;
    let first_compartment = &rucksack[..mid];
    let second_compartment = &rucksack[mid..];

    // Since there are only 52 different items in total we use a static array instead of a set.
    // +1 is needed for the `0` we use to model invalid input and error cases
    let mut appeared = [false; priority(b'Z') as usize + 1];

    for &item in first_compartment {
        appeared[priority(item) as usize] = true;
    }

    second_compartment
        .iter()
        .copied()
        .find(|item| appeared[priority(*item) as usize])
        .unwrap_or(0)
}

struct LineStream<R> {
    input: R,
    buffer: Vec<u8>,
}

impl<R> LineStream<R> {
    fn new(input: R) -> Self {
        Self {
            input,
            buffer: Vec::new(),
        }
    }

    fn next(&mut self) -> Option<&'_ [u8]>
    where
        R: BufRead,
    {
        self.buffer.clear();
        self.input.read_until(b'\n', &mut self.buffer).ok()?;
        if self.buffer.is_empty() {
            None
        } else {
            self.buffer.pop();
            Some(&self.buffer)
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::{common, priority, sum_of_priorities};

    #[test]
    fn common_priority() {
        assert_eq!(b'p', common(b"vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(b'L', common(b"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!(b'P', common(b"PmmdzqPrVvPwwTWBwg"));
        assert_eq!(b'v', common(b"wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!(b't', common(b"ttgJtRGJQctTZtZT"));
        assert_eq!(b's', common(b"CrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn priorities() {
        assert_eq!(16, priority(b'p'));
        assert_eq!(38, priority(b'L'));
        assert_eq!(42, priority(b'P'));
        assert_eq!(22, priority(b'v'));
        assert_eq!(20, priority(b't'));
        assert_eq!(19, priority(b's'));
    }

    #[test]
    fn rucksack_sum_of_priorities() {
        let input = "\
           vJrwpWtwJgWrhcsFMMfFFhFp\n\
           jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
           PmmdzqPrVvPwwTWBwg\n\
           wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
           ttgJtRGJQctTZtZT\n\
           CrZsJsPPZsGzwwsLwLmpwMDw\n\
        ";

        let actual = sum_of_priorities(Cursor::new(input));

        assert_eq!(157, actual)
    }
}
