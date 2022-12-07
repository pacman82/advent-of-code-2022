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
