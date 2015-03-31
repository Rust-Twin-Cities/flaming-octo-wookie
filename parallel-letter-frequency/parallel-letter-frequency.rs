#![feature(std_misc)]

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::Future;

/// Compute the frequency of each letter (technically of each unicode codepoint) using the given
/// number of worker threads.
pub fn frequency(texts: &[&str], num_workers: usize) -> HashMap<char, usize> {
    // ... 
}

