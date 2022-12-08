//! --- Day 7: No Space Left On Device ---
//!
//! You can hear birds chirping and raindrops hitting leaves as the expedition proceeds.
//! Occasionally, you can even hear much louder sounds in the distance; how big do the animals get
//! out here, anyway? The device the Elves gave you has problems with more than just its
//! communication system. You try to run a system update:
//!
//! ```
//! $ system-update --please --pretty-please-with-sugar-on-top
//! Error: No space left on device
//! ```
//!
//! Perhaps you can delete some files to make space for the update?
//! You browse around the filesystem to assess the situation and save the resulting terminal output
//! (your puzzle input). For example:
//!
//! ```
//! $ cd /
//! $ ls
//! dir a
//! 14848514 b.txt
//! 8504156 c.dat
//! dir d
//! $ cd a
//! $ ls
//! dir e
//! 29116 f
//! 2557 g
//! 62596 h.lst
//! $ cd e
//! $ ls
//! 584 i
//! $ cd ..
//! $ cd ..
//! $ cd d
//! $ ls
//! 4060174 j
//! 8033020 d.log
//! 5626152 d.ext
//! 7214296 k
//! ```
//!
//! The filesystem consists of a tree of files (plain data) and directories (which can contain other
//! directories or files). The outermost directory is called /. You can navigate around the
//! filesystem, moving into or out of directories and listing the contents of the directory you're
//! currently in. Within the terminal output, lines that begin with $ are commands you executed,
//! very much like some modern computers:
//!
//!
//! * cd means change directory. This changes which directory is the current directory, but the
//!   specific result depends on the argument:
//!   * cd x moves in one level: it looks in the current directory for the directory named x and
//!     makes it the current directory.
//!   * cd .. moves out one level: it finds the directory that contains the current directory, then
//!     makes that directory the current directory.
//!   * cd / switches the current directory to the outermost directory, /.
//! * ls means list. It prints out all of the files and directories immediately contained by the
//!   current directory:
//!   * 123 abc means that the current directory contains a file named abc with size 123.
//!   * dir xyz means that the current directory contains a directory named xyz.
//!
//! Given the commands and output in the example above, you can determine that the filesystem looks
//! visually like this:
//!
//! ```
//! - / (dir)
//!   - a (dir)
//!     - e (dir)
//!       - i (file, size=584)
//!     - f (file, size=29116)
//!     - g (file, size=2557)
//!     - h.lst (file, size=62596)
//!   - b.txt (file, size=14848514)
//!   - c.dat (file, size=8504156)
//!   - d (dir)
//!     - j (file, size=4060174)
//!     - d.log (file, size=8033020)
//!     - d.ext (file, size=5626152)
//!     - k (file, size=7214296)
//! ```
//!
//! Here, there are four directories: / (the outermost directory), a and d (which are in /), and e
//! (which is in a). These directories also contain files of various sizes. Since the disk is full,
//! your first step should probably be to find directories that are good candidates for deletion. To
//! do this, you need to determine the total size of each directory. The total size of a directory
//! is the sum of the sizes of the files it contains, directly or indirectly. (Directories
//! themselves do not count as having any intrinsic size.) The total sizes of the directories above
//! can be found as follows:
//!
//! * The total size of directory e is 584 because it contains a single file i of size 584 and no
//!   other directories.
//! * The directory a has total size 94853 because it contains files f (size 29116), g (size 2557),
//!   and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
//! * Directory d has total size 24933642.
//! * As the outermost directory, / contains every file. Its total size is 48381165, the sum of the
//!   size of every file.
//!
//! To begin, find all of the directories with a total size of at most 100000, then calculate the
//! sum of their total sizes. In the example above, these directories are a and e; the sum of their
//! total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than
//! once!) Find all of the directories with a total size of at most 100000.
//!
//! **What is the sum of the total sizes of those directories?**
//!
//! --- Part Two ---
//!
//! Now, you're ready to choose a directory to delete.
//!
//! The total disk space available to the filesystem is 70000000. To run the update, you need unused
//! space of at least 30000000. You need to find a directory you can delete that will free up enough
//! space to run the update. In the example above, the total size of the outermost directory (and
//! thus the total amount of used space) is 48381165; this means that the size of the unused space
//! must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore,
//! the update still requires a directory with total size of at least 8381165 to be deleted before
//! it can run. To achieve this, you have the following options:
//!
//! * Delete directory e, which would increase unused space by 584.
//! * Delete directory a, which would increase unused space by 94853.
//! * Delete directory d, which would increase unused space by 24933642.
//! * Delete directory /, which would increase unused space by 48381165.
//!
//! Directories e and a are both too small; deleting them would not free up enough space. However,
//! directories d and / are both big enough! Between these, choose the smallest: d, increasing
//! unused space by 24933642. Find the smallest directory that, if deleted, would free up enough
//! space on the filesystem to run the update. **What is the total size of that directory?**

use atoi::FromRadix10Checked;
use lines::LineStream;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod lines;

fn main() {
    let input = BufReader::new(File::open("input.txt").expect("Can not open input file"));
    let size = accumulated_directory_size(input);
    println!("{size}");
}

fn accumulated_directory_size(input: impl BufRead) -> u64 {
    let lines = LineStream::new(input);
    let mut terminal_output = TerminalOutput::new(lines);
    let first_command = terminal_output.next();
    assert_eq!(Some(Log::ToRoot), first_command);
    rec_directory_size(&mut terminal_output).accumulated_size
}

struct AggregatedSize {
    child_size: u64,
    accumulated_size: u64,
}

// The text does not specify this, but the input is a straight forward depth first search, so we
// won't keep track of any directory names
fn rec_directory_size(to: &mut TerminalOutput<impl BufRead>) -> AggregatedSize {
    let list = to.next();
    assert_eq!(Some(Log::Ls), list);
    let mut directory_size = 0;
    let mut accumulated_size = 0;
    while let Some(log) = to.next() {
        match log {
            Log::ToRoot | Log::Ls => panic!("yagni"),
            Log::ToChild => {
                let agg_size = rec_directory_size(to);
                directory_size += agg_size.child_size;
                accumulated_size += agg_size.accumulated_size;
            }
            Log::ToParent => break,
            Log::Directory => (),
            Log::File(size) => directory_size += size,
        }
    }
    if directory_size <= 100_000 {
        accumulated_size += directory_size;
    }
    AggregatedSize {
        child_size: directory_size,
        accumulated_size,
    }
}

struct TerminalOutput<R> {
    lines: LineStream<R>,
}

impl<R> TerminalOutput<R> {
    fn new(lines: LineStream<R>) -> Self {
        Self { lines }
    }

    fn next(&mut self) -> Option<Log>
    where
        R: BufRead,
    {
        self.lines.next().map(Log::from_line)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Log {
    ToRoot,
    ToChild,
    ToParent,
    Ls,
    Directory,
    File(u64),
}

impl Log {
    fn from_line(line: &[u8]) -> Self {
        if line.starts_with(b"$") {
            let command = &line[2..];
            if command == b"ls" {
                Log::Ls
            } else {
                assert!(command.starts_with(b"cd"));
                let to = &command[3..];
                match to {
                    b"/" => Log::ToRoot,
                    b".." => Log::ToParent,
                    _ => Log::ToChild,
                }
            }
        } else if line.starts_with(b"dir") {
            Log::Directory
        } else {
            let (size, _num_bytes) = u64::from_radix_10_checked(line);
            Log::File(size.expect("Expected file size. Found {line}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{accumulated_directory_size, lines::LineStream, Log, TerminalOutput};

    const TERMINAL_OUTPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

    #[test]
    fn parse_terminal_output() {
        let input = LineStream::new(Cursor::new(TERMINAL_OUTPUT));

        let mut to = TerminalOutput::new(input);
        assert_eq!(Some(Log::ToRoot), to.next());
        assert_eq!(Some(Log::Ls), to.next());
        assert_eq!(Some(Log::Directory), to.next());
        assert_eq!(Some(Log::File(14848514)), to.next());
        assert_eq!(Some(Log::File(8504156)), to.next());
        assert_eq!(Some(Log::Directory), to.next());
        assert_eq!(Some(Log::ToChild), to.next());
        assert_eq!(Some(Log::Ls), to.next());
        assert_eq!(Some(Log::Directory), to.next());
        assert_eq!(Some(Log::File(29116)), to.next());
        assert_eq!(Some(Log::File(2557)), to.next());
        assert_eq!(Some(Log::File(62596)), to.next());
        assert_eq!(Some(Log::ToChild), to.next());
        assert_eq!(Some(Log::Ls), to.next());
        assert_eq!(Some(Log::File(584)), to.next());
        assert_eq!(Some(Log::ToParent), to.next());
        assert_eq!(Some(Log::ToParent), to.next());
        assert_eq!(Some(Log::ToChild), to.next());
        assert_eq!(Some(Log::Ls), to.next());
        assert_eq!(Some(Log::File(4060174)), to.next());
        assert_eq!(Some(Log::File(8033020)), to.next());
        assert_eq!(Some(Log::File(5626152)), to.next());
        assert_eq!(Some(Log::File(7214296)), to.next());
        assert!(to.next().is_none());
    }

    #[test]
    fn accumulate_directory_sizes() {
        let input = Cursor::new(TERMINAL_OUTPUT);
        assert_eq!(95437, accumulated_directory_size(input));
    }
}
