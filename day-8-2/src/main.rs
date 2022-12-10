//! --- Day 8: Treetop Tree House ---
//!
//! The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The
//! Elves explain that a previous expedition planted these trees as a reforestation effort. Now,
//! they're curious if this would be a good location for a tree house. First, determine whether
//! there is enough tree cover here to keep a tree house hidden. To do this, you need to count the
//! number of trees that are visible from outside the grid when looking directly along a row or
//! column. The Elves have already launched a quadcopter to generate a map with the height of each
//! tree (your puzzle input). For example:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! Each tree is represented as a single digit whose value is its height, where 0 is the shortest
//! and 9 is the tallest. A tree is visible if all of the other trees between it and an edge of the
//! grid are shorter than it. Only consider trees in the same row or column; that is, only look up,
//! down, left, or right from any given tree. All of the trees around the edge of the grid are
//! visible - since they are already on the edge, there are no trees to block the view. In this
//! example, that only leaves the interior nine trees to consider:
//!
//! * The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom
//!   since other trees of height 5 are in the way.)
//! * The top-middle 5 is visible from the top and right.
//! * The top-right 1 is not visible from any direction; for it to be visible, there would need to
//!   only be trees of height 0 between it and an edge.
//! * The left-middle 5 is visible, but only from the right.
//! * The center 3 is not visible from any direction; for it to be visible, there would need to be
//!   only trees of at most height 2 between it and an edge.
//! * The right-middle 3 is visible from the right.
//! * In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
//!
//! With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are
//! visible in this arrangement. Consider your map; **how many trees are visible from outside the
//! grid?**
//!
//! --- Part Two ---
//!
//! Content with the amount of tree cover available, the Elves just need to know the best spot to
//! build their tree house: they would like to be able to see a lot of trees. To measure the viewing
//! distance from a given tree, look up, down, left, and right from that tree; stop if you reach an
//! edge or at the first tree that is the same height or taller than the tree under consideration.
//! (If a tree is right on the edge, at least one of its viewing distances will be zero.)
//!
//! The Elves don't care about distant trees taller than those found by the rules above; the
//! proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than
//! the tree house anyway. In the example above, consider the middle 5 in the second row:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! * Looking up, its view is not blocked; it can see 1 tree (of height 3).
//! * Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next
//!   to it).
//! * Looking right, its view is not blocked; it can see 2 trees.
//! * Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the
//!   tree of height 5 that blocks its view).
//!
//! A tree's scenic score is found by multiplying together its viewing distance in each of the four
//! directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).
//!
//! However, you can do even better: consider the tree of height 5 in the middle of the fourth row:
//!
//! ```
//! 30373
//! 25512
//! 65332
//! 33549
//! 35390
//! ```
//!
//! * Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
//! * Looking left, its view is not blocked; it can see 2 trees.
//! * Looking down, its view is also not blocked; it can see 1 tree.
//! * Looking right, its view is blocked at 2 trees (by a massive tree of height 9).
//!
//! This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.
//!
//! Consider each tree on your map. **What is the highest scenic score possible for any tree?**

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
    let num_visible_trees = grid.best_scenic_score();
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
        let mut extend_grid =
            |line: &[u8]| tree_heights.extend(line.iter().map(|ascii_digit| *ascii_digit - b'0'));
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

    /// (up, left, down, right)
    fn view_range(&self, index: usize) -> (usize, usize, usize, usize) {
        let row = index / self.width;
        let col = index % self.width;

        let current_height = self.tree_heights[index];

        let fuse_blocked_view = |last_tree_height: &mut Option<u8>, coord| {
            let this_tree_height = self[coord];
            if let Some(lth) = *last_tree_height {
                *last_tree_height = Some(this_tree_height);
                (current_height > lth).then_some(())
            } else {
                *last_tree_height = Some(this_tree_height);
                Some(())
            }
        };

        let up = (0..row)
            .rev()
            .map(|r| (r, col))
            .scan(None, fuse_blocked_view)
            .count();
        let down = ((row + 1)..self.height)
            .map(|r| (r, col))
            .scan(None, fuse_blocked_view)
            .count();
        let left = (0..col)
            .rev()
            .map(|c| (row, c))
            .scan(None, fuse_blocked_view)
            .count();
        let right = ((col + 1)..self.width)
            .map(|c| (row, c))
            .scan(None, fuse_blocked_view)
            .count();

        (up, left, down, right)
    }

    fn scenic_score(&self, index: usize) -> usize {
        let (up, left, down, right) = self.view_range(index);
        up * left * down * right
    }

    fn best_scenic_score(&self) -> usize {
        (0..self.tree_heights.len())
            .map(|index| self.scenic_score(index))
            .max()
            .unwrap()
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
    fn view_range() {
        let input = Cursor::new(INPUT);

        let grid = Grid::new(input);
        let ranges = |r: usize, c: usize| grid.view_range(5 * r + c);

        assert_eq!((1, 1, 2, 2), ranges(1, 2));
        assert_eq!((2, 2, 1, 2), ranges(3, 2));
    }

    #[test]
    fn scenic_scores() {
        let input = Cursor::new(INPUT);

        let grid = Grid::new(input);
        let score = |r: usize, c: usize| grid.scenic_score(5 * r + c);

        assert_eq!(4, score(1, 2));
        assert_eq!(8, score(3, 2));
    }

    #[test]
    fn max_scenic_score() {
        let input = Cursor::new(INPUT);

        let grid = Grid::new(input);
        let best = grid.best_scenic_score();

        assert_eq!(8, best);
    }
}
