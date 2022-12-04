//! --- Day 1: Calorie Counting ---
//!
//! Santa's reindeer typically eat regular reindeer food, but they need a lot of magical energy to
//! deliver presents on Christmas. For that, their favorite snack is a special type of star fruit
//! that only grows deep in the jungle. The Elves have brought you on their annual expedition to the
//! grove where the fruit grows. To supply enough magical energy, the expedition needs to retrieve a
//! minimum of fifty stars by December 25th. Although the Elves assure you that the grove has plenty
//! of fruit, you decide to grab any fruit you see along the way, just in case. Collect stars by
//! solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the
//! second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//! The jungle must be too overgrown and difficult to navigate in vehicles or access from the air;
//! the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin
//! taking inventory of their supplies. One important consideration is food - in particular, the
//! number of Calories each Elf is carrying (your puzzle input). The Elves take turns writing down
//! the number of Calories contained by the various meals, snacks, rations, etc. that they've
//! brought with them, one item per line. Each Elf separates their own inventory from the previous
//! Elf's inventory (if any) by a blank line.
//!
//! For example, suppose the Elves finish writing their items' Calories and end up with the
//! following list:
//!
//! 1000
//! 2000
//! 3000
//!
//! 4000
//!
//! 5000
//! 6000
//!
//! 7000
//! 8000
//! 9000
//!
//! 10000
//!
//! This list represents the Calories of the food carried by five Elves:
//!
//!     The first Elf is carrying food with 1000, 2000, and 3000 Calories, a total of 6000 Calories.
//!     The second Elf is carrying one food item with 4000 Calories.
//!     The third Elf is carrying food with 5000 and 6000 Calories, a total of 11000 Calories.
//!     The fourth Elf is carrying food with 7000, 8000, and 9000 Calories, a total of 24000
//! Calories.     The fifth Elf is carrying one food item with 10000 Calories.
//!
//! In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd
//! like to know how many Calories are being carried by the Elf carrying the most Calories. In the
//! example above, this is 24000 (carried by the fourth Elf). Find the Elf carrying the most
//! Calories. How many total Calories is that Elf carrying?

use anyhow::Error;
use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let input = BufReader::new(input);
    let max_calories = max_calories_in_list(input)?;
    println!("{max_calories}");
    Ok(())
}

/// The amount of calories the elf with the most calories is carrying
fn max_calories_in_list(mut list: impl BufRead) -> Result<u64, Error> {
    let mut buffer = String::new();

    // Most calories found so far
    let mut max_calories = 0;
    // Calories aggregated for the current elf so far
    let mut elf_calories = 0;

    loop {
        list.read_line(&mut buffer)?;
        // `pop` is a bug, in case the last elf in the input does not have a line break.
        if buffer.pop().is_none() {
            // End of file (EOF)
            break;
        } else if buffer.is_empty() {
            // Next elf
            max_calories = max(max_calories, elf_calories);
            elf_calories = 0;
        } else {
            let calories: u64 = buffer.parse()?;
            elf_calories += calories;
        }
        buffer.clear();
    }
    max_calories = max(max_calories, elf_calories);
    Ok(max_calories)
}

#[cfg(test)]
mod tests {
    use crate::max_calories_in_list;
    use std::io::Cursor;

    #[test]
    fn example_given_in_instruction() {
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
            10000";
        let input = Cursor::new(input);

        let actual = max_calories_in_list(input).unwrap();

        assert_eq!(24_000, actual);
    }
}
