use crate::io::{IoResult, Read, Write};

#[derive(Copy, Clone, Default)]
pub struct StdOut;

impl Write for StdOut {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Default)]
pub struct StdIn;

impl Read for StdIn {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        Ok(0)
    }
}
