//! --- Part Two ---
//!
//! By the time you calculate the answer to the Elves' question, they've already realized that the
//! Elf carrying the most Calories of food might eventually run out of snacks. To avoid this
//! unacceptable situation, the Elves would instead like to know the total Calories carried by the
//! top three Elves carrying the most Calories. That way, even if one of those Elves runs out of
//! snacks, they still have two backups. In the example above, the top three Elves are the fourth
//! Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with
//! 10000 Calories). The sum of the Calories carried by these three elves is 45000. Find the top
//! three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

use anyhow::Error;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let input = BufReader::new(input);
    let top3 = calories_top_3(input);
    println!("{top3}");
    Ok(())
}

/// Iterate over the aggregated amount of calories carried by each elf
fn elf_calories(mut list: impl BufRead) -> impl Iterator<Item = u64> {
    let mut buffer = String::new();

    iter::from_fn(move || {
        buffer.clear();
        list.read_line(&mut buffer).ok()?;
        if buffer.is_empty() {
            // We assume each line ends with a linebreak, even the last calories for the last elf.
            // If The line is "empty", as in it does not even contain a linebreak, than
            // we consider our input to be finished.
            None
        } else {
            // Calories aggregated for the current elf so far
            let mut elf_calories = 0;
            Some(loop {
                // Drop the linebreak at the end, if it exists.
                buffer.pop();
                if buffer.is_empty() {
                    // Just a line break or empty line => End of Elf
                    break elf_calories;
                } else {
                    let calories: u64 = buffer.parse().ok()?;
                    elf_calories += calories;
                    buffer.clear();
                    list.read_line(&mut buffer).ok()?;
                }
            })
        }
    })
}

struct Top([u64; 3]);

impl Top {
    fn new() -> Self {
        Self([0, 0, 0])
    }

    // Replace the minimum element of top3 with calories, if calories is larger.
    fn update(&mut self, calories: u64) {
        let (index, &min_cal) = self
            .0
            .iter()
            .enumerate()
            .min_by_key(|(_, cal)| **cal)
            .unwrap();
        self.0[index] = max(min_cal, calories)
    }
}

/// The amount of calories the elf with the most calories is carrying
fn calories_top_3(list: impl BufRead) -> u64 {
    let top = elf_calories(list).fold(Top::new(), |mut top, current| {
        top.update(current);
        top
    });
    top.0.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::{calories_top_3, elf_calories};

    #[test]
    fn elfs() {
        let input = "1000\n\
            2000\n\
            3000\n\
            \n\
            4000\n\
            \n\
            5000\n\
            6000\n\
            \n\
            7000\n\
            8000\n\
            9000\n\
            \n\
            10000\n"
            .as_bytes();

        let calories: Vec<u64> = elf_calories(input).collect();

        assert_eq!([6000, 4000, 11_000, 24_000, 10_000].as_slice(), &calories);
    }

    #[test]
    fn accumulate_top_3() {
        let input = "1000\n\
            2000\n\
            3000\n\
            \n\
            4000\n\
            \n\
            5000\n\
            6000\n\
            \n\
            7000\n\
            8000\n\
            9000\n\
            \n\
            10000\n"
            .as_bytes();

        let actual = calories_top_3(input);

        assert_eq!(45_000, actual);
    }
}
