//! --- Day 14: Regolith Reservoir ---
//!
//! The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like
//! it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a
//! little path that leads behind the waterfall. Correction: the distress signal leads you behind a
//! giant waterfall! There seems to be a large cave system here, and the signal definitely leads
//! further inside.
//!
//! As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand
//! begins pouring into the cave! If you don't quickly figure out where the sand is going, you could
//! quickly become trapped! Fortunately, your familiarity with analyzing the path of falling
//! material will come in handy here. You scan a two-dimensional vertical slice of the cave above
//! you (your puzzle input) and discover that it is mostly air with structures made of rock.
//! Your scan traces the path of each solid rock structure and reports the x,y coordinates that form
//! the shape of the path, where x represents distance to the right and y represents distance down.
//! Each path appears as a single line of text in your scan. After the first point of each path,
//! each point indicates the end of a straight horizontal or vertical line to be drawn from the
//! previous point. For example:
//!
//! ```
//! 498,4 -> 498,6 -> 496,6
//! 503,4 -> 502,4 -> 502,9 -> 494,9
//! ```
//!
//! This scan means that there are two paths of rock; the first path consists of two straight lines,
//! and the second path consists of three straight lines. (Specifically, the first path consists of
//! a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)
//! The sand is pouring into the cave from point 500,0.
//! Drawing rock as #, air as ., and the source of the sand as +, this becomes:
//!
//! ```
//!   4     5  5
//!   9     0  0
//!   4     0  3
//! 0 ......+...
//! 1 ..........
//! 2 ..........
//! 3 ..........
//! 4 ....#...##
//! 5 ....#...#.
//! 6 ..###...#.
//! 7 ........#.
//! 8 ........#.
//! 9 #########.
//! ```
//!
//! Sand is produced one unit at a time, and the next unit of sand is not produced until the
//! previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in
//! your scan. A unit of sand always falls down one step if possible. If the tile immediately below
//! is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down
//! and to the left. If that tile is blocked, the unit of sand attempts to instead move diagonally
//! one step down and to the right. Sand keeps moving as long as it is able to do so, at each step
//! trying to move down, then down-left, then down-right. If all three possible destinations are
//! blocked, the unit of sand comes to rest and no longer moves, at which point the next unit of
//! sand is created back at the source. So, drawing sand that has come to rest as o, the first unit
//! of sand simply falls straight down and then stops:
//!
//! ```
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ........#.
//! ......o.#.
//! #########.
//! ```
//!
//! The second unit of sand then falls straight down, lands on the first one, and then comes to rest
//! to its left:
//!
//! ```
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ........#.
//! .....oo.#.
//! #########.
//! ```
//!
//! After a total of five units of sand have come to rest, they form this pattern:
//!
//! ```
//! ......+...
//! ..........
//! ..........
//! ..........
//! ....#...##
//! ....#...#.
//! ..###...#.
//! ......o.#.
//! ....oooo#.
//! #########.
//! ```
//!
//! After a total of 22 units of sand:
//!
//! ```
//! ......+...
//! ..........
//! ......o...
//! .....ooo..
//! ....#ooo##
//! ....#ooo#.
//! ..###ooo#.
//! ....oooo#.
//! ...ooooo#.
//! #########.
//! ```
//!
//! Finally, only two more units of sand can possibly come to rest:
//!
//! ```
//! ......+...
//! ..........
//! ......o...
//! .....ooo..
//! ....#ooo##
//! ...o#ooo#.
//! ..###ooo#.
//! ....oooo#.
//! .o.ooooo#.
//! #########.
//! ```
//!
//! Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom,
//! falling into the endless void. Just for fun, the path any new sand takes before falling forever
//! is shown here with ~:
//!
//! ```
//! .......+...
//! .......~...
//! ......~o...
//! .....~ooo..
//! ....~#ooo##
//! ...~o#ooo#.
//! ..~###ooo#.
//! ..~..oooo#.
//! .~o.ooooo#.
//! ~#########.
//! ~..........
//! ~..........
//! ~..........
//! ```
//!
//! Using your scan, simulate the falling sand. How many units of sand come to rest before sand
//! starts flowing into the abyss below?

use atoi::FromRadix10Checked;
use common::LineStream;
use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_CAVE_WIDTH: usize = 1000;
const MAX_CAVE_HEIGHT: usize = 200;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("Can not open input file"));
    let mut cave = Cave::from_input(input);
    let sand_count = cave.fill_with_sand();
    println!("{sand_count}")
}

struct Cave {
    space: Vec<Field>,
}

impl Cave {
    fn from_input(input: impl BufRead) -> Self {
        let mut lines = LineStream::new(input);
        let mut space = vec![Field::Air; MAX_CAVE_HEIGHT * MAX_CAVE_WIDTH];

        while let Some(line) = lines.next_line() {
            let path = RockPath::from_line(line);
            for (x, y) in path.flatten() {
                space[y * MAX_CAVE_WIDTH + x] = Field::Rock;
            }
        }

        Self { space }
    }

    fn fill_with_sand(&mut self) -> usize {
        let mut sand_count = 0;
        while self.drop_sand_bulk() {
            sand_count += 1;
        }
        sand_count
    }

    /// `true` if sands lands and `false` if it goes into the abyss
    fn drop_sand_bulk(&mut self) -> bool {
        let mut pos = 500;
        loop {
            if pos / MAX_CAVE_WIDTH == MAX_CAVE_HEIGHT - 1 {
                break false;
            }
            let down = pos + MAX_CAVE_WIDTH;
            if self.space[down] == Field::Air {
                pos = down;
            } else if self.space[down - 1] == Field::Air {
                pos = down - 1;
            } else if self.space[down + 1] == Field::Air {
                pos = down + 1;
            } else {
                // Sand lands
                self.space[pos] = Field::Sand;
                break true;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Air,
    Rock,
    Sand,
}

struct RockPath<'a> {
    start: (i32, i32),
    line: &'a [u8],
}

impl<'a> RockPath<'a> {
    fn from_line(line: &'a [u8]) -> Self {
        let (start, line) = Self::split_off_coordinates(line);
        Self { line, start }
    }

    fn split_off_coordinates(line: &'a [u8]) -> ((i32, i32), &'a [u8]) {
        let (x, digits_x) = i32::from_radix_10_checked(line);
        let x = x.expect("Expected number");
        let (y, digits_y) = i32::from_radix_10_checked(&line[digits_x + 1..]);
        let y = y.expect("Expected number");
        let offset = min(digits_x + digits_y + 1 + " -> ".len(), line.len());
        assert!(x < MAX_CAVE_WIDTH as i32, "x = {x}");
        assert!(y < MAX_CAVE_HEIGHT as i32, "y = {y}");
        ((x, y), &line[offset..])
    }
}

impl Iterator for RockPath<'_> {
    type Item = Segment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.is_empty() {
            None
        } else {
            let (end, remainder) = Self::split_off_coordinates(self.line);
            let segment = Segment::new(self.start, end);
            self.start = end;
            self.line = remainder;
            Some(segment)
        }
    }
}

struct Segment {
    current: (i32, i32),
    end: (i32, i32),
}

impl Segment {
    fn new(a: (i32, i32), b: (i32, i32)) -> Self {
        let start = (min(a.0, b.0), min(a.1, b.1));
        let end = (max(a.0, b.0), max(a.1, b.1));
        let end = if start.0 == end.0 {
            (end.0, end.1 + 1)
        } else {
            (end.0 + 1, end.1)
        };
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for Segment {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let (x, y) = self.current;
            self.current = if self.current.0 == self.end.0 {
                (self.current.0, self.current.1 + 1)
            } else {
                (self.current.0 + 1, self.current.1)
            };
            Some((x as usize, y as usize))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Cave;

    #[test]
    fn sample_cave() {
        let input = "\
            498,4 -> 498,6 -> 496,6\n\
            503,4 -> 502,4 -> 502,9 -> 494,9\n\
        ";

        let mut cave = Cave::from_input(input.as_bytes());
        let sand_count = cave.fill_with_sand();

        assert_eq!(24, sand_count);
    }
}
