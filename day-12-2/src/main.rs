//! --- Day 12: Hill Climbing Algorithm ---
//!
//! You try contacting the Elves using your handheld device, but the river you're following must be
//! too low to get a decent signal. You ask the device for a heightmap of the surrounding area (your
//! puzzle input). The heightmap shows the local area from above broken into a grid; the elevation
//! of each square of the grid is given by a single lowercase letter, where a is the lowest
//! elevation, b is the next-lowest, and so on up to the highest elevation, z. Also included on the
//! heightmap are marks for your current position (S) and the location that should get the best
//! signal (E). Your current position (S) has elevation a, and the location that should get the best
//! signal (E) has elevation z. You'd like to reach E, but to save energy, you should do it in as
//! few steps as possible. During each step, you can move exactly one square up, down, left, or
//! right. To avoid needing to get out your climbing gear, the elevation of the destination square
//! can be at most one higher than the elevation of your current square; that is, if your current
//! elevation is m, you could step to elevation n, but not to elevation o. (This also means that the
//! elevation of the destination square can be much lower than the elevation of your current
//! square.)
//!
//! For example:
//!
//! ```
//! Sabqponm
//! abcryxxl
//! accszExk
//! acctuvwj
//! abdefghi
//! ```
//!
//! Here, you start in the top-left corner; your goal is near the middle. You could start by moving
//! down or right, but eventually you'll need to head toward the e at the bottom. From there, you
//! can spiral around to the goal:
//!
//! ```
//! v..v<<<<
//! >v.vv<<^
//! .>vv>E^^
//! ..v>>>^^
//! ..>>>>>^
//! ```
//!
//! In the above diagram, the symbols indicate whether the path exits each square moving up (^),
//! down (v), left (<), or right (>). The location that should get the best signal is still E, and .
//! marks unvisited squares. This path reaches the goal in 31 steps, the fewest possible.
//! What is the fewest steps required to move from your current position to the location that should
//! get the best signal?

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use common::LineStream;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("Can not open input file"));
    let grid = Grid::new(input);
    let dtg = grid.distance_to_goal();
    println!("{dtg}");
}

struct Grid {
    width: usize,
    heights: Vec<u8>,
    end: usize,
}

impl Grid {
    fn new(input: impl BufRead) -> Self {
        let mut lines = LineStream::new(input);
        let first = lines.next_line().expect("Grid must not be empty");
        let width = first.len();
        let mut heights = Vec::new();
        let mut start = 0;
        let mut end = 0;

        let locate = |what, within: &[u8], target: &mut usize, offset: usize| {
            if let Some(pos) = within.iter().position(|byte| *byte == what) {
                *target = offset + pos;
            }
        };

        heights.extend(first.iter().copied().map(byte_to_height));
        while let Some(line) = lines.next_line() {
            locate(b'S', line, &mut start, heights.len());
            locate(b'E', line, &mut end, heights.len());
            heights.extend(line.iter().copied().map(byte_to_height));
        }
        Grid {
            width,
            heights,
            end,
        }
    }

    fn distance_to_goal(&self) -> u32 {
        // Nothing fancy, breadth first search
        let mut open = HashSet::new();
        open.insert(self.end);
        let mut closed: HashSet<usize> = HashSet::new();
        let mut neighbours = Vec::new();
        let mut steps = 0;
        loop {
            if neighbours.iter().any(|index| self.heights[*index] == 0) {
                break;
            }
            closed.extend(&open);
            neighbours.clear();
            for &pos in &open {
                self.extend_neighbours(pos, &mut neighbours);
            }
            open.clear();
            open.extend(&neighbours);
            steps += 1;
        }
        steps
    }

    fn extend_neighbours(&self, index: usize, neighbours: &mut Vec<usize>) {
        let grid_height = self.heights.len() / self.width;
        let x = (index % self.width) as isize;
        let y = (index / self.width) as isize;
        let current_height = self.heights[index];
        neighbours.extend(
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter(|(a, b)| {
                    *a >= 0 && *b >= 0 && *a < self.width as isize && *b < grid_height as isize
                })
                .map(|(a, b)| *b as usize * self.width + *a as usize)
                .filter(|n| self.heights[*n] + 1 >= current_height),
        );
    }
}

fn byte_to_height(byte: u8) -> u8 {
    match byte {
        h @ b'a'..=b'z' => h - b'a',
        b'S' => 0,
        b'E' => b'z' - b'a',
        _ => panic!("Invalid height"),
    }
}
