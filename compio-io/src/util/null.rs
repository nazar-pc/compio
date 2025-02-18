use compio_buf::{BufResult, IoBufMut};

use crate::{AsyncBufRead, AsyncRead, AsyncWrite, IoResult};

/// An empty reader and writer constructed via [`null`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Null {
    _p: (),
}

impl AsyncRead for Null {
    async fn read<B: IoBufMut>(&mut self, buf: B) -> compio_buf::BufResult<usize, B> {
        BufResult(Ok(0), buf)
    }
}

impl AsyncBufRead for Null {
    async fn fill_buf(&mut self) -> IoResult<&'_ [u8]> {
        Ok(&[])
    }

    fn consume(&mut self, _: usize) {}
}

impl AsyncWrite for Null {
    async fn write<T: compio_buf::IoBuf>(&mut self, buf: T) -> BufResult<usize, T> {
        BufResult(Ok(0), buf)
    }

    async fn write_vectored<T: compio_buf::IoVectoredBuf>(
        &mut self,
        buf: T,
    ) -> BufResult<usize, T> {
        BufResult(Ok(0), buf)
    }

    async fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> IoResult<()> {
        Ok(())
    }
}

/// Create a new [`Null`] reader and writer which acts like a black hole.
///
/// All reads from and writes to this reader will return
/// [`BufResult(Ok(0), buf)`] and leave the buffer unchanged.
///
/// # Examples
///
/// ```
/// use compio_io::{null, AsyncRead, AsyncWrite};
///
/// # #[compio_macros::main] async fn main() {
/// let mut buf = Vec::with_capacity(10);
/// let mut null = null();
///
/// let (num_read, buf) = null.read(buf).await.unwrap();
///
/// assert_eq!(num_read, 0);
/// assert!(buf.is_empty());
///
/// let (num_written, buf) = null.write(buf).await.unwrap();
/// assert_eq!(num_written, 0);
/// # }
/// ```
///
/// [`BufResult(Ok(0), buf)`]: compio_buf::BufResult
#[inline(always)]
pub fn null() -> Null {
    Null { _p: () }
}
