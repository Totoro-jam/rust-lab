//! Integration tests for ownership and borrowing concepts.

use rustlab02::smart_pointers::SharedCounter;
use rustlab02::text_buffer::{longest, Excerpt, TextBuffer};

#[test]
fn text_buffer_borrow_and_mutate() {
    let mut buf = TextBuffer::new(String::from("rust"));
    // Immutable borrow
    assert_eq!(buf.as_str(), "rust");
    // Mutable borrow
    buf.append(" rocks");
    assert_eq!(buf.as_str(), "rust rocks");
}

#[test]
fn text_buffer_clone_independence() {
    let buf1 = TextBuffer::new(String::from("original"));
    let mut buf2 = buf1.clone();
    buf2.append(" modified");

    assert_eq!(buf1.as_str(), "original");
    assert_eq!(buf2.as_str(), "original modified");
}

#[test]
fn lifetime_annotations() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("hi");
        result = longest(s1.as_str(), s2.as_str());
        assert_eq!(result, "long string");
    }
    // result is valid here because it references s1 which outlives the block
}

#[test]
fn excerpt_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt = Excerpt::new(&novel[..16]);
    assert_eq!(excerpt.text, "Call me Ishmael.");
    assert_eq!(excerpt.first_word(), "Call");
}

#[test]
fn shared_counter_multiple_owners() {
    let c1 = SharedCounter::new(10);
    let c2 = c1.clone_handle();
    let c3 = c1.clone_handle();

    assert_eq!(c1.strong_count(), 3);

    c2.increment();
    c3.increment();

    assert_eq!(c1.get(), 12);
    drop(c3);
    assert_eq!(c1.strong_count(), 2);
}
