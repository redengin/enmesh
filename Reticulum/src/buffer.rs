//! Buffer types for reading and writing data in Reticulum.
//!
//! This module provides low-level buffer abstractions for handling fixed-size
//! and dynamic byte buffers used throughout Reticulum for packet construction,
//! data serialization, and network I/O operations.
//!
//! # Overview
//!
//! The buffer module provides three main types:
//! - [`StaticBuffer`]: A fixed-size buffer with compile-time known capacity
//! - [`OutputBuffer`]: A dynamic buffer for writing data to an external slice
//! - [`InputBuffer`]: A read-only buffer for parsing data from a slice
//!
//! These types are used internally by other Reticulum modules for packet
//! construction, cryptographic operations, and interface communication.

use core::cmp::min;
use core::fmt;

use crate::error::RnsError;

/// A fixed-size buffer with a compile-time determined capacity.
///
/// StaticBuffer provides a stack-allocated buffer with a maximum capacity
/// determined at compile time via the const generic parameter `N`. It is
/// designed for scenarios where the maximum buffer size is known ahead of
/// time, such as packet headers or fixed protocol messages.
///
/// # Usage
///
/// StaticBuffer is created with a specific capacity and automatically tracks
/// the current length of data written to it:
///
/// ```
/// use reticulum::buffer::StaticBuffer;
///
/// let mut buffer: StaticBuffer<256> = StaticBuffer::new();
/// buffer.write(b"Hello, Reticulum!").unwrap();
/// assert_eq!(buffer.len(), 17);
/// ```
///
/// # Memory Layout
///
/// The buffer consists of a fixed-size array of `N` bytes and a `len` field
/// that tracks the current write position. The unused portion of the buffer
/// is left uninitialized for performance reasons.
///
/// # Thread Safety
///
/// StaticBuffer does not implement any synchronization primitives. It is
/// the caller's responsibility to ensure exclusive access when modifying
/// the buffer from multiple threads.
///
/// # Example: Chaining Write Operations
///
/// StaticBuffer supports method chaining for convenient multiple writes:
///
/// ```
/// use reticulum::buffer::StaticBuffer;
///
/// let mut buffer: StaticBuffer<512> = StaticBuffer::new();
/// buffer.chain_write(b"header:").expect("no more buffer");
/// buffer.chain_write(b"payload").expect("no more buffer");
/// buffer.chain_safe_write(b"extra");
///
/// assert_eq!(buffer.as_slice(), b"header:payloadextra");
/// ```
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct StaticBuffer<const N: usize> {
    buffer: [u8; N],
    len: usize,
}
impl<const N: usize> StaticBuffer<N> {
    /// Creates a new empty StaticBuffer with zero length.
    ///
    /// The buffer is allocated on the stack with all bytes initially zeroed.
    /// The buffer starts empty (length 0) and data must be written to it
    /// using the `write` or `safe_write` methods.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let buffer: StaticBuffer<128> = StaticBuffer::new();
    /// assert_eq!(buffer.len(), 0);
    /// ```
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; N],
            len: 0,
        }
    }

    /// Creates a new StaticBuffer initialized with data from a slice.
    ///
    /// If the provided slice is larger than the buffer capacity `N`, only
    /// the first `N` bytes are written. This is a convenience constructor
    /// that combines allocation and initial writing.
    ///
    /// # Arguments
    ///
    /// * `data` - The initial data to populate the buffer with
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let buffer: StaticBuffer<12> = StaticBuffer::new_from_slice(b"initial data");
    /// assert_eq!(buffer.len(), 12);
    /// assert_eq!(buffer.as_slice(), b"initial data");
    /// ```
    pub fn new_from_slice(data: &[u8]) -> Self {
        let mut buffer = Self::new();

        buffer.safe_write(data);

        buffer
    }

    /// Resets the buffer to zero length, discarding all written data.
    ///
    /// This method does not zero the underlying memory - it simply resets
    /// the length counter to 0. The previous data remains in the buffer
    /// but is considered overwritten on the next write operation.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<4> = StaticBuffer::new_from_slice(b"data");
    /// buffer.reset();
    /// assert_eq!(buffer.len(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.len = 0;
    }

    /// Resizes the buffer to a new length.
    ///
    /// If the requested length is greater than the buffer capacity `N`,
    /// the buffer is truncated to `N`. This allows shrinking the effective
    /// length of the buffer without reallocating.
    ///
    /// # Arguments
    ///
    /// * `len` - The new length for the buffer
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<11> = StaticBuffer::new_from_slice(b"hello world");
    /// buffer.resize(5);
    /// assert_eq!(buffer.len(), 5);
    /// assert_eq!(buffer.as_slice(), b"hello");
    /// ```
    pub fn resize(&mut self, len: usize) {
        self.len = min(len, self.buffer.len());
    }

    /// Returns the current length of data in the buffer.
    ///
    /// This is the number of bytes that have been successfully written to
    /// the buffer, not the total capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let buffer: StaticBuffer<4> = StaticBuffer::new_from_slice(b"test");
    /// assert_eq!(buffer.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Writes data to the buffer and returns self for chaining.
    ///
    /// This method attempts to write the entire data slice to the buffer.
    /// If the write would exceed the buffer capacity, an error is returned
    /// and no data is written.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Returns
    ///
    /// * `Ok(&mut Self)` - Returns self if the write succeeded for chaining
    /// * `Err(RnsError::OutOfMemory)` - If the data would exceed buffer capacity
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<64> = StaticBuffer::new();
    /// //buffer.chain_write(b"part1").chain_write(b"part2").unwrap();
    /// buffer.chain_write(b"part1").expect("no more buffer");
    /// buffer.chain_write(b"part2").expect("no more buffer");
    /// assert_eq!(buffer.as_slice(), b"part1part2");
    /// ```
    pub fn chain_write(&mut self, data: &[u8]) -> Result<&mut Self, RnsError> {
        self.write(data)?;
        Ok(self)
    }

    /// Returns the current buffer state (identity function).
    ///
    /// This method exists to provide a consistent API but simply returns
    /// self. It can be useful in generic contexts or for method chaining
    /// consistency.
    pub fn finalize(self) -> Self {
        self
    }

    /// Writes data to the buffer without returning errors.
    ///
    /// Unlike `write()`, this method silently truncates the data if it
    /// would exceed the buffer capacity, writing only as many bytes as
    /// will fit. The number of bytes actually written is returned.
    ///
    /// This is useful when the caller knows the data might be too large
    /// and wants to ensure at least some data is written without error handling.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Returns
    ///
    /// The number of bytes actually written (may be less than data.len())
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<10> = StaticBuffer::new();
    /// let written = buffer.safe_write(b"this is too long");
    /// assert_eq!(written, 10);  // Only 10 bytes fit
    /// ```
    pub fn safe_write(&mut self, data: &[u8]) -> usize {
        let data_size = data.len();

        let max_size = core::cmp::min(data_size, N - self.len);

        self.write(&data[..max_size]).unwrap_or(0)
    }

    /// Writes data to the buffer using safe semantics for chaining.
    ///
    /// Similar to `safe_write()` but returns self for method chaining.
    /// The write is truncated if it would exceed capacity.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<20> = StaticBuffer::new();
    /// buffer.chain_safe_write(b"hello").chain_safe_write(b" world");
    /// assert_eq!(buffer.as_slice(), b"hello world");
    /// ```
    pub fn chain_safe_write(&mut self, data: &[u8]) -> &mut Self {
        self.safe_write(data);
        self
    }

    /// Writes data to the buffer.
    ///
    /// Attempts to write the entire data slice to the buffer starting at
    /// the current position. The buffer's length is updated to reflect
    /// the new data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of bytes written
    /// * `Err(RnsError::OutOfMemory)` - If the data would exceed buffer capacity
    /// * `Err(RnsError::InvalidArgument)` - If data is empty (nothing to write)
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<64> = StaticBuffer::new();
    /// let written = buffer.write(b"Hello").unwrap();
    /// assert_eq!(written, 5);
    /// ```
    pub fn write(&mut self, data: &[u8]) -> Result<usize, RnsError> {
        let data_size = data.len();

        // Nothing to write
        if data_size == 0 {
            return Ok(0);
        }

        if (self.len + data_size) > N {
            return Err(RnsError::OutOfMemory);
        }

        self.buffer[self.len..(self.len + data_size)].copy_from_slice(data);
        self.len += data_size;

        Ok(data_size)
    }

    // FIXME description and example don't match the behavior
    // /// Rotates the buffer left by `mid` bytes.
    // ///
    // /// This moves the data at the beginning of the buffer to the end,
    // /// effectively discarding the first `mid` bytes and shifting the
    // /// remaining data to the start. Useful for protocols that have
    // /// variable-length headers where you need to "consume" the header
    // /// and keep the payload.
    // ///
    // /// # Arguments
    // ///
    // /// * `mid` - The number of bytes to rotate from the start to the end
    // ///
    // /// # Returns
    // ///
    // /// * `Ok(usize)` - The new length after rotation
    // /// * `Err(RnsError::InvalidArgument)` - If mid > current buffer length
    // ///
    // /// # Example
    // ///
    // /// ```ignore
    // /// use reticulum::buffer::StaticBuffer;
    // ///
    // /// let mut buffer: StaticBuffer<6> = StaticBuffer::new_from_slice(b"ABCDEF");
    // /// buffer.rotate_left(3).unwrap();  // "ABC" moved to end
    // /// assert_eq!(buffer.as_slice(), b"DEFABC");
    // /// ```
    // pub fn rotate_left(&mut self, mid: usize) -> Result<usize, RnsError> {
    //     if mid > self.len {
    //         return Err(RnsError::InvalidArgument);
    //     }

    //     self.len -= mid;

    //     self.buffer.rotate_left(mid);

    //     Ok(self.len)
    // }

    /// Returns a read-only slice of the valid data in the buffer.
    ///
    /// The returned slice has length equal to `self.len()` and contains
    /// all bytes from position 0 to the current write position.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let buffer: StaticBuffer<4> = StaticBuffer::new_from_slice(b"data");
    /// let slice: &[u8] = buffer.as_slice();
    /// assert_eq!(slice, b"data");
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.len]
    }

    /// Returns a mutable slice of the valid data in the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<4> = StaticBuffer::new_from_slice(b"data");
    /// buffer.as_mut_slice()[0] = b'D';  // Modify first byte
    /// assert_eq!(buffer.as_slice(), b"Data");
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.len]
    }

    /// Acquires a buffer for writing with an exact length.
    ///
    /// This provides direct access to the underlying buffer for bulk writing.
    /// The buffer's length is set to the specified value, allowing you to
    /// write exactly `len` bytes to the returned slice.
    ///
    /// # Arguments
    ///
    /// * `len` - The number of bytes to acquire space for
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<128> = StaticBuffer::new();
    /// let buf = buffer.accuire_buf(64);
    /// assert_eq!(buffer.len(), 64);
    /// ```
    pub fn accuire_buf(&mut self, len: usize) -> &mut [u8] {
        self.len = len;
        &mut self.buffer[..self.len]
    }

    /// Acquires the maximum buffer capacity for writing.
    ///
    /// This is a convenience method that acquires the entire buffer capacity
    /// for writing. The buffer's length is set to the maximum capacity `N`.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::StaticBuffer;
    ///
    /// let mut buffer: StaticBuffer<64> = StaticBuffer::new();
    /// let buf = buffer.accuire_buf_max();
    /// // buf now has 64 bytes of writable space
    /// assert_eq!(buffer.len(), 64);
    /// ```
    pub fn accuire_buf_max(&mut self) -> &mut [u8] {
        self.len = self.buffer.len();
        &mut self.buffer[..self.len]
    }
}

impl<const N: usize> Default for StaticBuffer<N> {
    fn default() -> Self {
        Self {
            buffer: [0u8; N],
            len: 0,
        }
    }
}

/// A dynamic buffer for writing data to an external slice.
///
/// OutputBuffer provides a view over an externally-owned mutable slice,
/// tracking the current write position (offset). It is useful when you
/// have a pre-allocated buffer and want a safe interface for writing
/// data to it.
///
/// # Lifetime
///
/// The lifetime parameter `'a` ties the OutputBuffer to the lifetime of
/// the underlying slice it wraps. The buffer must not outlive the data
/// it references.
///
/// # Example
///
/// ```
/// use reticulum::buffer::OutputBuffer;
///
/// let mut data = [0u8; 64];
/// let mut buf = OutputBuffer::new(&mut data);
/// buf.write(b"Hello").unwrap();
/// buf.write(b" World").unwrap();
/// assert_eq!(buf.offset(), 11);
/// assert_eq!(buf.as_slice(), b"Hello World");
/// ```
pub struct OutputBuffer<'a> {
    buffer: &'a mut [u8],
    offset: usize,
}
impl<'a> OutputBuffer<'a> {
    /// Creates a new OutputBuffer wrapping an external slice.
    ///
    /// The buffer starts at offset 0, ready for writing from the beginning
    /// of the provided slice.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The mutable slice to wrap
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::OutputBuffer;
    ///
    /// let mut data = [0u8; 100];
    /// let buf = OutputBuffer::new(&mut data);
    /// assert_eq!(buf.offset(), 0);
    /// ```
    pub fn new(buffer: &'a mut [u8]) -> Self {
        Self { offset: 0, buffer }
    }

    /// Writes data to the buffer.
    ///
    /// Attempts to write the entire data slice to the buffer starting at
    /// the current offset. The offset is updated to reflect the new position.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to write to the buffer
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of bytes written
    /// * `Err(RnsError::OutOfMemory)` - If the data would exceed buffer capacity
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::OutputBuffer;
    ///
    /// let mut data = [0u8; 32];
    /// let mut buf = OutputBuffer::new(&mut data);
    /// let written = buf.write(b"test data").unwrap();
    /// assert_eq!(written, 9);
    /// ```
    pub fn write(&mut self, data: &[u8]) -> Result<usize, RnsError> {
        let data_size = data.len();

        // Nothing to write
        if data_size == 0 {
            return Ok(0);
        }

        if (self.offset + data_size) > self.buffer.len() {
            return Err(RnsError::OutOfMemory);
        }

        self.buffer[self.offset..(self.offset + data_size)].copy_from_slice(data);
        self.offset += data_size;

        Ok(data_size)
    }

    /// Writes a single byte to the buffer.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte value to write
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - Always returns 1 (one byte written)
    /// * `Err(RnsError::OutOfMemory)` - If the buffer is full
    pub fn write_byte(&mut self, byte: u8) -> Result<usize, RnsError> {
        self.write(&[byte])
    }

    /// Resets the offset to zero, allowing the buffer to be overwritten.
    ///
    /// This does not zero the underlying memory - it simply resets the
    /// write position to the beginning.
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Returns true if the buffer is completely filled.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::OutputBuffer;
    ///
    /// let mut data = [0u8; 5];
    /// let mut buf = OutputBuffer::new(&mut data);
    /// assert!(!buf.is_full());
    /// buf.write(b"hello").unwrap();
    /// assert!(buf.is_full());
    /// ```
    pub fn is_full(&self) -> bool {
        self.offset == self.buffer.len()
    }

    /// Returns the current write offset in the buffer.
    ///
    /// This is the number of bytes that have been written so far.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns a read-only slice of the data written so far.
    ///
    /// The returned slice has length equal to `self.offset()` and contains
    /// all bytes from position 0 to the current write position.
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[..self.offset]
    }

    /// Returns a mutable slice of the data written so far.
    ///
    /// This allows direct modification of the written data.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[..self.offset]
    }
}

impl<'a> fmt::Display for OutputBuffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ 0x")?;

        for i in 0..self.offset {
            write!(f, "{:0>2x}", self.buffer[i])?;
        }

        write!(f, " ]",)
    }
}

impl<const N: usize> fmt::Display for StaticBuffer<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ 0x")?;

        for i in 0..self.len {
            write!(f, "{:0>2x}", self.buffer[i])?;
        }

        write!(f, " ]",)
    }
}

/// A read-only buffer for parsing data from a slice.
///
/// InputBuffer provides a view over an immutable slice with a read cursor,
/// making it useful for parsing binary protocols or extracting structured
/// data from raw bytes.
///
/// # Lifetime
///
/// The lifetime parameter `'a` ties the InputBuffer to the lifetime of
/// the underlying slice it reads from. The buffer must not outlive the
/// data it references.
///
/// # Example
///
/// ```
/// use reticulum::buffer::InputBuffer;
///
/// let data = b"Hello World";
/// let mut buf = InputBuffer::new(data);
/// let hello = buf.read_slice(5).unwrap();
/// assert_eq!(hello, b"Hello");
/// assert_eq!(buf.bytes_left(), 6);
/// ```
pub struct InputBuffer<'a> {
    buffer: &'a [u8],
    offset: usize,
}
impl<'a> InputBuffer<'a> {
    /// Creates a new InputBuffer wrapping a slice.
    ///
    /// The buffer starts at offset 0, ready for reading from the beginning
    /// of the provided slice.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The immutable slice to wrap
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { offset: 0, buffer }
    }

    /// Reads data from the buffer into the provided buffer.
    ///
    /// Reads exactly `buf.len()` bytes from the current offset, advancing
    /// the cursor by that amount.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to read into (length determines bytes to read)
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of bytes read
    /// * `Err(RnsError::OutOfMemory)` - If not enough bytes remain in the buffer
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::InputBuffer;
    ///
    /// let data = b"Hello World";
    /// let mut buf = InputBuffer::new(data);
    /// let mut dest = [0u8; 5];
    /// buf.read(&mut dest).unwrap();
    /// assert_eq!(&dest, b"Hello");
    /// ```
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, RnsError> {
        let size = buf.len();
        if (self.offset + size) > self.buffer.len() {
            return Err(RnsError::OutOfMemory);
        }

        buf.copy_from_slice(&self.buffer[self.offset..(self.offset + size)]);
        self.offset += size;

        Ok(size)
    }

    /// Reads a specific number of bytes from the buffer.
    ///
    /// Similar to `read()`, but allows specifying an exact byte count
    /// that may differ from the destination buffer's length.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to read into
    /// * `size` - The exact number of bytes to read
    ///
    /// # Returns
    ///
    /// * `Ok(usize)` - The number of bytes read
    /// * `Err(RnsError::OutOfMemory)` - If not enough bytes remain or buf is too small
    pub fn read_size(&mut self, buf: &mut [u8], size: usize) -> Result<usize, RnsError> {
        if (self.offset + size) > self.buffer.len() {
            return Err(RnsError::OutOfMemory);
        }

        if buf.len() < size {
            return Err(RnsError::OutOfMemory);
        }

        buf[..size].copy_from_slice(&self.buffer[self.offset..(self.offset + size)]);
        self.offset += size;

        Ok(size)
    }

    /// Reads a single byte from the buffer.
    ///
    /// # Returns
    ///
    /// * `Ok(u8)` - The byte value read
    /// * `Err(RnsError::OutOfMemory)` - If no bytes remain in the buffer
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::InputBuffer;
    ///
    /// let data = b"ABC";
    /// let mut buf = InputBuffer::new(data);
    /// let byte = buf.read_byte().unwrap();
    /// assert_eq!(byte, b'A');
    /// ```
    pub fn read_byte(&mut self) -> Result<u8, RnsError> {
        let mut buf = [0u8; 1];
        self.read(&mut buf)?;

        Ok(buf[0])
    }

    /// Reads a slice of exactly `size` bytes from the buffer.
    ///
    /// Unlike `read()` which copies into a provided buffer, this method
    /// returns a direct reference to the underlying data. This is more
    /// efficient but the returned slice is tied to the InputBuffer's lifetime.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to read
    ///
    /// # Returns
    ///
    /// * `Ok(&[u8])` - A slice containing exactly `size` bytes
    /// * `Err(RnsError::OutOfMemory)` - If not enough bytes remain in the buffer
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::InputBuffer;
    ///
    /// let data = b"Hello World";
    /// let mut buf = InputBuffer::new(data);
    /// let hello = buf.read_slice(5).unwrap();
    /// assert_eq!(hello, b"Hello");
    /// ```
    pub fn read_slice(&mut self, size: usize) -> Result<&[u8], RnsError> {
        if (self.offset + size) > self.buffer.len() {
            return Err(RnsError::OutOfMemory);
        }

        let slice = &self.buffer[self.offset..self.offset + size];

        self.offset += size;

        Ok(slice)
    }

    /// Returns the number of bytes remaining in the buffer.
    ///
    /// This is the total buffer length minus the current offset.
    ///
    /// # Example
    ///
    /// ```
    /// use reticulum::buffer::InputBuffer;
    ///
    /// let data = b"Hello";
    /// let mut buf = InputBuffer::new(data);
    /// assert_eq!(buf.bytes_left(), 5);
    /// buf.read_slice(2).unwrap();
    /// assert_eq!(buf.bytes_left(), 3);
    /// ```
    pub fn bytes_left(&self) -> usize {
        self.buffer.len() - self.offset
    }
}
