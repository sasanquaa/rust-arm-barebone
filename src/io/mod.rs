use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

pub mod device;
pub mod fs;
pub mod std;

#[derive(Copy, Clone, Debug)]
pub enum IoError {}

impl Display for IoError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl Error for IoError {}

type IoResult<T> = Result<T, IoError>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize>;

    fn flush(&mut self) -> IoResult<()>;
}

pub trait Ioctl<R> {
    fn ioctl(&mut self, req: R) -> IoResult<()>;
}
