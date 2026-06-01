//! Efficient byte manipulation with the `bytes` crate.
//!
//! Demonstrates Bytes (immutable), BytesMut (mutable),
//! zero-copy slicing, and Buf/BufMut traits.

use bytes::{Buf, BufMut, Bytes, BytesMut};

/// Creates a Bytes buffer from a string and returns its length.
pub fn from_static_str(s: &'static str) -> Bytes {
    Bytes::from_static(s.as_bytes())
}

/// Splits a BytesMut at a given position (zero-copy).
pub fn split_buffer(data: &[u8], at: usize) -> (Bytes, Bytes) {
    let mut buf = BytesMut::from(data);
    let right = buf.split_off(at);
    (buf.freeze(), right.freeze())
}

/// Demonstrates the BufMut trait: building a buffer incrementally.
pub fn build_message(id: u32, payload: &[u8]) -> Bytes {
    let mut buf = BytesMut::with_capacity(4 + payload.len());
    buf.put_u32(id);
    buf.put_slice(payload);
    buf.freeze()
}

/// Reads a u32 header + payload from a Bytes buffer using the Buf trait.
pub fn read_message(mut data: Bytes) -> Option<(u32, Bytes)> {
    if data.remaining() < 4 {
        return None;
    }
    let id = data.get_u32();
    Some((id, data))
}

/// Concatenates multiple byte slices into a single Bytes.
pub fn concat_buffers(parts: &[&[u8]]) -> Bytes {
    let total: usize = parts.iter().map(|p| p.len()).sum();
    let mut buf = BytesMut::with_capacity(total);
    for part in parts {
        buf.put_slice(part);
    }
    buf.freeze()
}

/// Demonstrates zero-copy slicing of Bytes.
pub fn slice_buffer(data: Bytes, start: usize, end: usize) -> Bytes {
    data.slice(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_bytes() {
        let b = from_static_str("hello");
        assert_eq!(&b[..], b"hello");
        assert_eq!(b.len(), 5);
    }

    #[test]
    fn split_and_verify() {
        let data = b"hello world";
        let (left, right) = split_buffer(data, 5);
        assert_eq!(&left[..], b"hello");
        assert_eq!(&right[..], b" world");
    }

    #[test]
    fn message_roundtrip() {
        let msg = build_message(42, b"payload");
        let (id, payload) = read_message(msg).unwrap();
        assert_eq!(id, 42);
        assert_eq!(&payload[..], b"payload");
    }

    #[test]
    fn read_too_short() {
        let data = Bytes::from_static(&[0, 1]);
        assert!(read_message(data).is_none());
    }

    #[test]
    fn concat_multiple() {
        let result = concat_buffers(&[b"foo", b"bar", b"baz"]);
        assert_eq!(&result[..], b"foobarbaz");
    }

    #[test]
    fn concat_empty() {
        let result = concat_buffers(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn slice_bytes() {
        let data = Bytes::from_static(b"hello world");
        let sliced = slice_buffer(data, 6, 11);
        assert_eq!(&sliced[..], b"world");
    }
}
