#[cfg(feature = "allocator_api")]
use std::alloc::Allocator;
use std::mem::MaybeUninit;

use crate::*;

/// A trait for buffers.
///
/// The `IoBuf` trait is implemented by buffer types that can be passed to
/// compio operations. Users will not need to use this trait directly.
pub trait IoBuf: Unpin + 'static {
    /// Returns a raw pointer to the vector’s buffer.
    ///
    /// This method is to be used by the `compio` runtime and it is not
    /// expected for users to call it directly.
    fn as_buf_ptr(&self) -> *const u8;

    /// Number of initialized bytes.
    ///
    /// This method is to be used by the `compio` runtime and it is not
    /// expected for users to call it directly.
    ///
    /// For [`Vec`], this is identical to `len()`.
    fn buf_len(&self) -> usize;

    /// Total size of the buffer, including uninitialized memory, if any.
    ///
    /// This method is to be used by the `compio` runtime and it is not
    /// expected for users to call it directly.
    ///
    /// For [`Vec`], this is identical to `capacity()`.
    fn buf_capacity(&self) -> usize;

    /// Get the initialized part of the buffer.
    fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.as_buf_ptr(), self.buf_len()) }
    }

    /// Create an [`IoSlice`] of this buffer.
    ///
    /// # Safety
    ///
    /// The return slice will not live longer than `Self`.
    /// It is static to provide convenience from writing self-referenced
    /// structure.
    unsafe fn as_io_slice(&self) -> IoSlice {
        IoSlice::from_slice(self.as_slice())
    }

    /// Returns a view of the buffer with the specified range.
    ///
    /// This method is similar to Rust's slicing (`&buf[..]`), but takes
    /// ownership of the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use compio_buf::IoBuf;
    ///
    /// let buf = b"hello world";
    /// assert_eq!(buf.slice(6..).as_slice(), b"world");
    /// ```
    fn slice(self, range: impl std::ops::RangeBounds<usize>) -> Slice<Self>
    where
        Self: Sized,
    {
        use std::ops::Bound;

        let begin = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        assert!(begin <= self.buf_capacity());

        let end = match range.end_bound() {
            Bound::Included(&n) => n.checked_add(1).expect("out of range"),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.buf_capacity(),
        };

        assert!(end <= self.buf_capacity());
        assert!(begin <= self.buf_len());

        Slice::new(self, begin, end)
    }

    /// Indicate wether the buffer has been filled (uninit portion is empty)
    fn filled(&self) -> bool {
        self.buf_len() == self.buf_capacity()
    }
}

impl<#[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoBuf for vec_alloc!(u8, A) {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.capacity()
    }
}

impl<#[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoBuf
    for box_alloc!([u8], A)
{
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

impl IoBuf for &'static mut [u8] {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

impl IoBuf for &'static [u8] {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

impl IoBuf for String {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.capacity()
    }
}

impl IoBuf for &'static mut str {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

impl IoBuf for &'static str {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> IoBuf for [u8; N] {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        N
    }

    fn buf_capacity(&self) -> usize {
        N
    }
}

#[cfg(feature = "bytes")]
impl IoBuf for bytes::Bytes {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "bytes")]
impl IoBuf for bytes::BytesMut {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.capacity()
    }
}

#[cfg(feature = "read_buf")]
impl IoBuf for std::io::BorrowedBuf<'static> {
    fn as_buf_ptr(&self) -> *const u8 {
        self.filled().as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.capacity()
    }
}

#[cfg(feature = "arrayvec")]
impl<const N: usize> IoBuf for arrayvec::ArrayVec<u8, N> {
    fn as_buf_ptr(&self) -> *const u8 {
        self.as_ptr()
    }

    fn buf_len(&self) -> usize {
        self.len()
    }

    fn buf_capacity(&self) -> usize {
        self.capacity()
    }
}

/// A mutable compio compatible buffer.
///
/// The `IoBufMut` trait is implemented by buffer types that can be passed to
/// compio operations. Users will not need to use this trait directly.
///
/// # Safety
///
/// Buffers passed to compio operations must reference a stable memory
/// region. While the runtime holds ownership to a buffer, the pointer returned
/// by `as_buf_mut_ptr` must remain valid even if the `IoBufMut` value is moved.
pub trait IoBufMut: IoBuf + SetBufInit {
    /// Returns a raw mutable pointer to the vector’s buffer.
    ///
    /// This method is to be used by the `compio` runtime and it is not
    /// expected for users to call it directly.
    fn as_buf_mut_ptr(&mut self) -> *mut u8;

    /// Get the uninitialized part of the buffer.
    fn as_mut_slice(&mut self) -> &mut [MaybeUninit<u8>] {
        unsafe { std::slice::from_raw_parts_mut(self.as_buf_mut_ptr().cast(), self.buf_capacity()) }
    }

    /// Create an [`IoSliceMut`] of the uninitialized part of the buffer.
    ///
    /// # Safety
    ///
    /// The return slice will not live longer than self.
    /// It is static to provide convenience from writing self-referenced
    /// structure.
    unsafe fn as_io_slice_mut(&mut self) -> IoSliceMut {
        IoSliceMut::from_uninit(self.as_mut_slice())
    }
}

impl<#[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoBufMut
    for vec_alloc!(u8, A)
{
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

impl IoBufMut for &'static mut [u8] {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

impl<const N: usize> IoBufMut for [u8; N] {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

#[cfg(feature = "bytes")]
impl IoBufMut for bytes::BytesMut {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

#[cfg(feature = "read_buf")]
impl IoBufMut for std::io::BorrowedBuf<'static> {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.filled().as_ptr() as _
    }
}

#[cfg(feature = "arrayvec")]
impl<const N: usize> IoBufMut for arrayvec::ArrayVec<u8, N> {
    fn as_buf_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_ptr()
    }
}

/// A trait for vectored buffers.
pub trait IoVectoredBuf: Unpin + 'static {
    /// An iterator for the [`IoSlice`]s of the buffers.
    ///
    /// # Safety
    ///
    /// The return slice will not live longer than self.
    /// It is static to provide convenience from writing self-referenced
    /// structure.
    unsafe fn as_io_slices(&self) -> Vec<IoSlice> {
        self.as_dyn_bufs().map(|buf| buf.as_io_slice()).collect()
    }

    /// Iterate the inner buffers.
    fn as_dyn_bufs(&self) -> impl Iterator<Item = &dyn IoBuf>;

    /// Create an owned iterator to make it easy to pass this vectored buffer as
    /// a regular buffer.
    ///
    /// ```
    /// use compio_buf::{IoBuf, IoVectoredBuf};
    ///
    /// let bufs = [vec![1u8, 2], vec![3, 4]];
    /// let iter = bufs.owned_iter().unwrap();
    /// assert_eq!(iter.as_slice(), &[1, 2]);
    /// let iter = iter.next().unwrap();
    /// assert_eq!(iter.as_slice(), &[3, 4]);
    /// let iter = iter.next();
    /// assert!(iter.is_err());
    /// ```
    ///
    /// The time complexity of the returned iterator depends on the
    /// implementation of [`Iterator::nth`] of [`IoVectoredBuf::as_dyn_bufs`].
    fn owned_iter(self) -> Result<OwnedIter<impl OwnedIterator<Inner = Self> + Unpin>, Self>
    where
        Self: Sized;
}

impl<T: IoBuf, const N: usize> IoVectoredBuf for [T; N] {
    fn as_dyn_bufs(&self) -> impl Iterator<Item = &dyn IoBuf> {
        self.iter().map(|buf| buf as &dyn IoBuf)
    }

    fn owned_iter(self) -> Result<OwnedIter<impl OwnedIterator<Inner = Self>>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

impl<T: IoBuf, #[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoVectoredBuf
    for vec_alloc!(T, A)
{
    fn as_dyn_bufs(&self) -> impl Iterator<Item = &dyn IoBuf> {
        self.iter().map(|buf| buf as &dyn IoBuf)
    }

    fn owned_iter(self) -> Result<OwnedIter<impl OwnedIterator<Inner = Self>>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

#[cfg(feature = "arrayvec")]
impl<T: IoBuf, const N: usize> IoVectoredBuf for arrayvec::ArrayVec<T, N> {
    fn as_dyn_bufs(&self) -> impl Iterator<Item = &dyn IoBuf> {
        self.iter().map(|buf| buf as &dyn IoBuf)
    }

    fn owned_iter(self) -> Result<OwnedIter<impl OwnedIterator<Inner = Self>>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

/// A trait for mutable vectored buffers.
pub trait IoVectoredBufMut: IoVectoredBuf + SetBufInit {
    /// An iterator for the [`IoSliceMut`]s of the buffers.
    ///
    /// # Safety
    ///
    /// The return slice will not live longer than self.
    /// It is static to provide convenience from writing self-referenced
    /// structure.
    unsafe fn as_io_slices_mut(&mut self) -> Vec<IoSliceMut> {
        self.as_dyn_mut_bufs()
            .map(|buf| buf.as_io_slice_mut())
            .collect()
    }

    /// Iterate the inner buffers.
    fn as_dyn_mut_bufs(&mut self) -> impl Iterator<Item = &mut dyn IoBufMut>;

    /// Create an owned iterator to make it easy to pass this vectored buffer as
    /// a regular buffer.
    ///
    /// ```
    /// use compio_buf::{IoBuf, IoVectoredBufMut};
    ///
    /// let bufs = [vec![1u8, 2], vec![3, 4]];
    /// let iter = bufs.owned_iter_mut().unwrap();
    /// assert_eq!(iter.as_slice(), &[1, 2]);
    /// let iter = iter.next().unwrap();
    /// assert_eq!(iter.as_slice(), &[3, 4]);
    /// let iter = iter.next();
    /// assert!(iter.is_err());
    /// ```
    ///
    /// The time complexity of the returned iterator depends on the
    /// implementation of [`Iterator::nth`] of [`IoVectoredBuf::as_dyn_bufs`].
    fn owned_iter_mut(self) -> Result<OwnedIter<impl OwnedIteratorMut<Inner = Self> + Unpin>, Self>
    where
        Self: Sized;
}

impl<T: IoBufMut, const N: usize> IoVectoredBufMut for [T; N] {
    fn as_dyn_mut_bufs(&mut self) -> impl Iterator<Item = &mut dyn IoBufMut> {
        self.iter_mut().map(|buf| buf as &mut dyn IoBufMut)
    }

    fn owned_iter_mut(self) -> Result<OwnedIter<impl OwnedIteratorMut<Inner = Self> + Unpin>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

impl<T: IoBufMut, #[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoVectoredBufMut
    for vec_alloc!(T, A)
{
    fn as_dyn_mut_bufs(&mut self) -> impl Iterator<Item = &mut dyn IoBufMut> {
        self.iter_mut().map(|buf| buf as &mut dyn IoBufMut)
    }

    fn owned_iter_mut(self) -> Result<OwnedIter<impl OwnedIteratorMut<Inner = Self>>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

#[cfg(feature = "arrayvec")]
impl<T: IoBufMut, const N: usize> IoVectoredBufMut for arrayvec::ArrayVec<T, N> {
    fn as_dyn_mut_bufs(&mut self) -> impl Iterator<Item = &mut dyn IoBufMut> {
        self.iter_mut().map(|buf| buf as &mut dyn IoBufMut)
    }

    fn owned_iter_mut(self) -> Result<OwnedIter<impl OwnedIteratorMut<Inner = Self>>, Self>
    where
        Self: Sized,
    {
        IndexedIter::new(self, 0).map(OwnedIter::new)
    }
}

/// A trait for vectored buffers that could be indexed.
pub trait IoIndexedBuf: IoVectoredBuf {
    /// Get the buffer with specific index.
    fn buf_nth(&self, n: usize) -> Option<&dyn IoBuf>;
}

impl<T: IoBuf, const N: usize> IoIndexedBuf for [T; N] {
    fn buf_nth(&self, n: usize) -> Option<&dyn IoBuf> {
        self.get(n).map(|b| b as _)
    }
}

impl<T: IoBuf, #[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoIndexedBuf
    for vec_alloc!(T, A)
{
    fn buf_nth(&self, n: usize) -> Option<&dyn IoBuf> {
        self.get(n).map(|b| b as _)
    }
}

#[cfg(feature = "arrayvec")]
impl<T: IoBuf, const N: usize> IoIndexedBuf for arrayvec::ArrayVec<T, N> {
    fn buf_nth(&self, n: usize) -> Option<&dyn IoBuf> {
        self.get(n).map(|b| b as _)
    }
}

/// A trait for mutable vectored buffers that could be indexed.
pub trait IoIndexedBufMut: IoVectoredBufMut + IoIndexedBuf {
    /// Get the mutable buffer with specific index.
    fn buf_nth_mut(&mut self, n: usize) -> Option<&mut dyn IoBufMut>;
}

impl<T: IoBufMut, const N: usize> IoIndexedBufMut for [T; N] {
    fn buf_nth_mut(&mut self, n: usize) -> Option<&mut dyn IoBufMut> {
        self.get_mut(n).map(|b| b as _)
    }
}

impl<T: IoBufMut, #[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> IoIndexedBufMut
    for vec_alloc!(T, A)
{
    fn buf_nth_mut(&mut self, n: usize) -> Option<&mut dyn IoBufMut> {
        self.get_mut(n).map(|b| b as _)
    }
}

#[cfg(feature = "arrayvec")]
impl<T: IoBufMut, const N: usize> IoIndexedBufMut for arrayvec::ArrayVec<T, N> {
    fn buf_nth_mut(&mut self, n: usize) -> Option<&mut dyn IoBufMut> {
        self.get_mut(n).map(|b| b as _)
    }
}

/// A helper trait for `set_len` like methods.
pub trait SetBufInit {
    /// Set the buffer length. If `len` is less than the current length, nothing
    /// should happen.
    ///
    /// # Safety
    ///
    /// `len` should be less or equal than `buf_capacity() - buf_len()`.
    unsafe fn set_buf_init(&mut self, len: usize);
}

impl<#[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> SetBufInit
    for vec_alloc!(u8, A)
{
    unsafe fn set_buf_init(&mut self, len: usize) {
        if self.buf_len() < len {
            self.set_len(len);
        }
    }
}

impl SetBufInit for &'static mut [u8] {
    unsafe fn set_buf_init(&mut self, len: usize) {
        debug_assert!(len <= self.len());
    }
}

impl<const N: usize> SetBufInit for [u8; N] {
    unsafe fn set_buf_init(&mut self, len: usize) {
        debug_assert!(len <= N);
    }
}

#[cfg(feature = "bytes")]
impl SetBufInit for bytes::BytesMut {
    unsafe fn set_buf_init(&mut self, len: usize) {
        if self.buf_len() < len {
            self.set_len(len);
        }
    }
}

#[cfg(feature = "read_buf")]
impl SetBufInit for std::io::BorrowedBuf<'static> {
    unsafe fn set_buf_init(&mut self, len: usize) {
        let current_len = self.buf_len();
        if current_len < len {
            self.unfilled().advance(len - current_len);
        }
    }
}

#[cfg(feature = "arrayvec")]
impl<const N: usize> SetBufInit for arrayvec::ArrayVec<u8, N> {
    unsafe fn set_buf_init(&mut self, len: usize) {
        if self.buf_len() < len {
            self.set_len(len);
        }
    }
}

impl<T: IoBufMut, const N: usize> SetBufInit for [T; N] {
    unsafe fn set_buf_init(&mut self, len: usize) {
        default_set_buf_init(self.iter_mut(), len)
    }
}

impl<T: IoBufMut, #[cfg(feature = "allocator_api")] A: Allocator + Unpin + 'static> SetBufInit
    for vec_alloc!(T, A)
{
    unsafe fn set_buf_init(&mut self, len: usize) {
        default_set_buf_init(self.iter_mut(), len)
    }
}

#[cfg(feature = "arrayvec")]
impl<T: IoBufMut, const N: usize> SetBufInit for arrayvec::ArrayVec<T, N> {
    unsafe fn set_buf_init(&mut self, len: usize) {
        default_set_buf_init(self.iter_mut(), len)
    }
}

unsafe fn default_set_buf_init<'a, B: IoBufMut>(
    iter: impl IntoIterator<Item = &'a mut B>,
    mut len: usize,
) {
    for buf in iter {
        let capacity = buf.buf_capacity();
        if len >= capacity {
            buf.set_buf_init(capacity);
            len -= capacity;
        } else {
            buf.set_buf_init(len);
            len = 0;
        }
    }
}
