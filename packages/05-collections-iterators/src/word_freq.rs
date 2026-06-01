//! Word frequency counter demonstrating HashMap, BTreeMap, iterators.

use std::collections::{BTreeMap, HashMap, HashSet};

/// Counts word frequency using iterator chains.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

/// Same logic using the entry API explicitly.
pub fn word_frequencies_entry_api(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let normalized = word
            .to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        if !normalized.is_empty() {
            *map.entry(normalized).or_insert(0) += 1;
        }
    }
    map
}

/// Returns top N words sorted by frequency (descending), using BTreeMap for stable ordering.
pub fn top_n(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut pairs: Vec<_> = freq.iter().map(|(k, &v)| (k.clone(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.into_iter().take(n).collect()
}

/// Returns words sorted alphabetically with their counts using BTreeMap.
pub fn sorted_frequencies(text: &str) -> BTreeMap<String, usize> {
    let freq = word_frequencies(text);
    freq.into_iter().collect()
}

/// Unique words in the text.
pub fn unique_words(text: &str) -> HashSet<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_frequency() {
        let text = "the cat sat on the mat the cat";
        let freq = word_frequencies(text);
        assert_eq!(freq["the"], 3);
        assert_eq!(freq["cat"], 2);
        assert_eq!(freq["sat"], 1);
    }

    #[test]
    fn top_n_words() {
        let text = "a a a b b c";
        let freq = word_frequencies(text);
        let top = top_n(&freq, 2);
        assert_eq!(top[0], ("a".to_string(), 3));
        assert_eq!(top[1], ("b".to_string(), 2));
    }

    #[test]
    fn unique_words_dedup() {
        let text = "hello Hello HELLO world";
        let unique = unique_words(text);
        assert_eq!(unique.len(), 2);
        assert!(unique.contains("hello"));
        assert!(unique.contains("world"));
    }

    #[test]
    fn sorted_frequencies_alphabetical() {
        let text = "banana apple cherry apple";
        let sorted = sorted_frequencies(text);
        let keys: Vec<_> = sorted.keys().collect();
        assert_eq!(keys, vec!["apple", "banana", "cherry"]);
    }
}
