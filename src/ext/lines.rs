// SPDX-License-Identifier: MPL-2.0
pub trait LinesOps {
    fn next_match<F>(&self, index: usize, predicate: F) -> Option<usize>
    where
        F: Fn(&String) -> bool;
    fn prev_match<F>(&self, index: usize, predicate: F) -> Option<usize>
    where
        F: Fn(&String) -> bool;
}

impl<V> LinesOps for V
where
    V: AsRef<[String]>,
{
    fn next_match<F>(&self, index: usize, predicate: F) -> Option<usize>
    where
        F: Fn(&String) -> bool,
    {
        self.as_ref()
            .iter()
            .skip(index)
            .position(predicate)
            .map(|pos| index + pos)
    }

    fn prev_match<F>(&self, index: usize, predicate: F) -> Option<usize>
    where
        F: Fn(&String) -> bool,
    {
        self.as_ref().iter().take(index).rposition(predicate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_match() {
        testing::with_trace(|| {
            let data: Vec<String> = ["0", "1", "1", "1", "0", "0", "1", "0", "1", "1", "1"]
                .iter()
                .map(|x| x.to_string())
                .collect();
            assert_eq!(data.next_match(0, |x| x == "1"), Some(1));
            assert_eq!(data.next_match(1, |x| x == "0"), Some(4));
            assert_eq!(data.prev_match(10, |x| x == "1"), Some(9));
            assert_eq!(data.prev_match(5, |x| x == "0"), Some(4));

            // out of bounds
            assert_eq!(data.next_match(11, |x| x == "0"), None);
            assert_eq!(data.next_match(11, |x| x == "1"), None);
            assert_eq!(data.prev_match(0, |x| x == "0"), None);
            assert_eq!(data.prev_match(0, |x| x == "1"), None);
            Ok(())
        });
    }
}
