use std::io::BufRead;

pub struct LineStream<R> {
    input: R,
    /// Number of lines to extract at once
    num_lines: usize,
    buffer: Vec<u8>,
}

impl<R> LineStream<R> {
    pub fn new(input: R) -> Self {
        Self::with_num_lines(input, 1)
    }

    pub fn with_num_lines(input: R, num_lines: usize) -> Self {
        Self {
            input,
            num_lines,
            buffer: Vec::new(),
        }
    }

    pub fn next_line(&mut self) -> Option<&'_ [u8]>
    where
        R: BufRead,
    {
        self.buffer.clear();
        for _ in 0..self.num_lines {
            self.input.read_until(b'\n', &mut self.buffer).ok()?;
        }
        if self.buffer.is_empty() {
            None
        } else {
            self.buffer.pop();
            Some(&self.buffer)
        }
    }
}

pub struct InputIterator<R, F> {
    lines: LineStream<R>,
    line_to_input: F,
}

impl<R, F> InputIterator<R, F> {
    pub fn new(read: R, line_to_input: F) -> Self {
        let lines = LineStream::new(read);
        Self {
            lines,
            line_to_input,
        }
    }
}

impl<R, F, I> Iterator for InputIterator<R, F>
where
    R: BufRead,
    F: FnMut(&[u8]) -> I,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next_line().map(|line| (self.line_to_input)(line))
    }
}
