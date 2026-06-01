//! A `TextBuffer` that owns a String, demonstrating ownership patterns.
//!
//! Methods show `&self`, `&mut self`, move semantics, and returning
//! references with explicit lifetime annotations.

/// A simple text buffer owning its content.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    content: String,
}

impl TextBuffer {
    /// Creates a new `TextBuffer` taking ownership of the given string.
    pub fn new(content: String) -> Self {
        Self { content }
    }

    /// Immutable borrow: returns a reference to the content.
    pub fn as_str(&self) -> &str {
        &self.content
    }

    /// Mutable borrow: appends text.
    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// Mutable borrow: clears the buffer.
    pub fn clear(&mut self) {
        self.content.clear();
    }

    /// Consumes self (move), returning the inner String.
    pub fn into_inner(self) -> String {
        self.content
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.content.len()
    }

    /// Returns whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Demonstrates lifetime annotations: the returned reference lives as long
/// as the shorter of the two input lifetimes.
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// A struct that borrows a string slice, requiring a lifetime parameter.
#[derive(Debug)]
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Lifetime elision: single input reference, output lifetime inferred.
    pub fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

/// Demonstrates Copy vs Clone: i32 is Copy, String is not.
pub fn demonstrate_copy_vs_move() -> (i32, String) {
    let x: i32 = 42;
    let y = x; // Copy: x still usable
    let _ = x + y;

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Must clone; s1 would be moved otherwise
    (y, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_basics() {
        let mut buf = TextBuffer::new(String::from("hello"));
        assert_eq!(buf.as_str(), "hello");

        buf.append(" world");
        assert_eq!(buf.as_str(), "hello world");
        assert_eq!(buf.len(), 11);
    }

    #[test]
    fn buffer_move_semantics() {
        let buf = TextBuffer::new(String::from("owned"));
        let inner = buf.into_inner();
        assert_eq!(inner, "owned");
        // buf is moved, cannot be used here
    }

    #[test]
    fn longest_returns_longer() {
        let a = "short";
        let b = "much longer";
        assert_eq!(longest(a, b), "much longer");
    }

    #[test]
    fn excerpt_borrows_correctly() {
        let text = String::from("hello world");
        let exc = Excerpt::new(&text);
        assert_eq!(exc.first_word(), "hello");
    }

    #[test]
    fn copy_vs_move() {
        let (y, s) = demonstrate_copy_vs_move();
        assert_eq!(y, 42);
        assert_eq!(s, "hello");
    }
}
