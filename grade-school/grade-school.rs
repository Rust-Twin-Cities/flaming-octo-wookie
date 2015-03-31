use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct School {
    // ...
}

impl School {
    pub fn new() -> School {
        // ...
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // ...
    }
    
    pub fn grades(&self) -> Vec<u32> {
        // ...
    }

    pub fn grade(&self, grade: u32) -> Option<&Vec<String>> {
        // ...
    }
}
