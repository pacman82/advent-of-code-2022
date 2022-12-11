use std::io::BufRead;

pub struct LineStream<R> {
    input: R,
    buffer: Vec<u8>,
}

impl<R> LineStream<R> {
    pub fn new(input: R) -> Self {
        Self {
            input,
            buffer: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<&'_ [u8]>
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
        self.lines.next().map(|line| (self.line_to_input)(line))
    }
}
