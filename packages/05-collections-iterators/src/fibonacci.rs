//! Custom Iterator: infinite Fibonacci sequence.

/// An infinite Fibonacci sequence iterator.
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(current)
    }
}

/// A range iterator that yields values within [start, end).
pub struct RangeStep {
    current: i64,
    end: i64,
    step: i64,
}

impl RangeStep {
    pub fn new(start: i64, end: i64, step: i64) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for RangeStep {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
            || self.step == 0
        {
            return None;
        }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_first_ten() {
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn fibonacci_with_filter() {
        let even_fibs: Vec<u64> = Fibonacci::new().take(15).filter(|n| n % 2 == 0).collect();
        assert_eq!(even_fibs, vec![0, 2, 8, 34, 144]);
    }

    #[test]
    fn fibonacci_sum() {
        let sum: u64 = Fibonacci::new().take(10).sum();
        assert_eq!(sum, 88);
    }

    #[test]
    fn range_step_basic() {
        let vals: Vec<i64> = RangeStep::new(0, 10, 3).collect();
        assert_eq!(vals, vec![0, 3, 6, 9]);
    }

    #[test]
    fn range_step_negative() {
        let vals: Vec<i64> = RangeStep::new(10, 0, -3).collect();
        assert_eq!(vals, vec![10, 7, 4, 1]);
    }
}
