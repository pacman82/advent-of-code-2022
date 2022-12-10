//! --- Day 8: Treetop Tree House ---
//!
//! The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.
//! First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.
//! The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
//! A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
//! All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:
//!
//! * The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
//! * The top-middle 5 is visible from the top and right.
//! * The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
//! * The left-middle 5 is visible, but only from the right.
//! * The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
//! * The right-middle 3 is visible from the right.
//! * In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
//!
//! With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.
//! Consider your map; **how many trees are visible from outside the grid?**

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Index,
};

use lines::LineStream;

mod lines;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("Can not open input file"));
    let grid = Grid::new(input);
    let num_visible_trees = grid.num_visible();
    println!("{num_visible_trees}");
}

struct Grid {
    width: usize,
    height: usize,
    tree_heights: Vec<u8>,
}

impl Grid {
    fn new(input: impl BufRead) -> Self {
        let mut lines = LineStream::new(input);
        let mut tree_heights = Vec::new();
        let first = lines.next().expect("Input must not be empty");
        let width = first.len();
        let mut extend_grid = |line: &[u8]| tree_heights.extend(line.iter().map(|ascii_digit| *ascii_digit - b'0'));
        extend_grid(first);
        while let Some(line) = lines.next() {
            assert_eq!(width, line.len());
            extend_grid(line)
        }
        let height = tree_heights.len() / width;
        Self {
            width,
            height,
            tree_heights,
        }
    }

    fn is_visible(&self, index: usize) -> bool {
        let row = index / self.width;
        let col = index % self.width;

        let current_height = self.tree_heights[index];

        let is_shorter = |(r, c)| self[(r, c)] < current_height;

        (0..row).map(|r| (r, col)).all(is_shorter)
            || ((row + 1)..self.height).map(|r| (r, col)).all(is_shorter)
            || (0..col).map(|c| (row, c)).all(is_shorter)
            || ((col + 1)..self.width).map(|c| (row, c)).all(is_shorter)
    }

    fn num_visible(&self) -> usize {
        (0..self.tree_heights.len()).filter(|index| self.is_visible(*index)).count()
    }
}

// Row, Column
impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.tree_heights[index.0 * self.width + index.1]
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::Grid;


    const INPUT: &str = "\
        30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390\n\
    ";

    #[test]
    fn visible_trees() {
        let input = Cursor::new(INPUT);

        let grid = Grid::new(input);
        let is_visible = |r: usize,c: usize| grid.is_visible(5 * r + c);

        assert!(is_visible(1,1));
        assert!(is_visible(1,2));
    }
}