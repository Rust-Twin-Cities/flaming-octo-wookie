use std::iter::{IntoIterator, FromIterator};
use std::slice;
use std::option::Option;
use std::vec::Vec;

#[derive(Debug)]
pub struct CustomSet<T> {
    // ...
}

/// Create an empty set.
impl <T> CustomSet<T> where T: Eq + Clone {
    pub fn new() -> CustomSet<T> {
        // ...
    }

    pub fn len(&self) -> usize {
        // ...
    }

    pub fn is_empty(&self) -> bool {
        // ...
    }

    pub fn contains(&self, needle: &T) -> bool {
        // ...
    }

    pub fn insert(&mut self, element: T) -> bool {
        // ...
    }

    pub fn remove(&mut self, needle: &T) -> bool {
        // ...
    }
    
    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        // ...
    }

    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        // ...
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        // ...
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        // ...
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        // ...
    }
    
    pub fn is_superset(&self, other: &CustomSet<T>) -> bool {
        // ...
    }

    pub fn clear(&mut self) {
        // ...
    }

    pub fn iter(&self) -> Iter<T>{
        // ...
    }
}

pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>
}

impl<'a, T> IntoIterator for &'a CustomSet<T> where T: Eq + Clone {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> FromIterator<T> for CustomSet<T> where T: Eq + Clone {
    fn from_iter<I: IntoIterator<Item=T>>(iterable: I) -> CustomSet<T> {
        // ...
    }
}
