// Copyright 2020 Rob King
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate provides a collection of data structures for storing sets of integers.
//! The different data structures are designed to make different operations efficient.
//!
//! All of the data structures in this crate support the following operations with the
//! associated complexity:
//!
//! - **contains** - check if an integer is in the set in O(1) time
//! - **iterate** - iterate over the members of the set in O(*n*) time, where *n* is the number of
//!                 elements in the set
//! - **len** - return the number of elements in the set in O(1) time
//!
//! Individual set data structures support additional operations, as documented below.
//!
//! All of the set data structures in this crate have a maximum capacity, specified as the
//! largest integer that can be stored in the set plus one. Once a set is created, it does
//! no further allocations.

/// A `GrowSet` is a set of integers that supports efficient addition and clearing.
///
/// It supports the following additional operations with the associated complexity:
///
/// - **add** - add an integer to the set in O(1) time
/// - **clear** - remove all members from the set in O(1) time
/// - **pop** - remove and return a random member of the set in O(1) time
///
/// `GrowSet`s are useful for sets that need to be cleared frequently and rebuilt.
///
/// The implementation of `GrowSet` is based on "An Efficient Represtation of Sparse Sets"
/// by Briggs and Torczon (1993).
#[derive(Debug)]
pub struct GrowSet {
    n: usize,
    sparse: Vec<usize>,
    dense: Vec<usize>,
}

impl<'a> GrowSet {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            n: 0,
            sparse: vec![0; size],
            dense: vec![0; size],
        }
    }

    #[inline(always)]
    pub fn contains(&self, value: usize) -> bool {
        self.sparse[value] < self.n && self.dense[self.sparse[value]] == value
    }

    /// Remove all items from the set.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.n = 0;
    }

    #[inline(always)]
    pub fn add(&mut self, value: usize) {
        if !self.contains(value) {
            self.dense[self.n] = value;
            self.sparse[value] = self.n;
            self.n += 1;
        }
    }

    /// Returns an iterator over the values in the set.
    /// Uniqueness is guaranteed; ordering is not.
    pub fn iter(&'a self) -> impl Iterator<Item = &usize> + 'a {
        self.dense.iter().take(self.n)
    }
}

/// A `ShrinkSet` is a set of integers that supports efficient removal and refilling.
///
/// `ShrinkSet`s are automatically initialized such that they contain all integers
/// up to, but not including, their capacity. For example, a `ShrinkSet` with a capacity
/// of 5 will contain the integers 0, 1, 2, 3, and 4 upon initialization.
///
/// A `ShrinkSet` supports the following additional operations with the associated
/// time complexity:
///
/// - **remove** - remove an integer from the set in O(1) time
/// - **refill** - adds all removed elements back into the set in O(1) time
/// - **pop** - remove and return a random member of the set in O(1) time
///
/// `ShrinkSet`s are useful for situations where we want to prune search spaces or
/// work queues (for example) and then reset them to their initial state efficiently.
///
/// The algorithm used by `ShrinkSet` is believed to be novel, but if there is existing
/// literature, please let me know so I can reference it here.
#[derive(Debug)]
pub struct ShrinkSet {
    p: usize,
    map: Vec<usize>,
    values: Vec<usize>,
}

impl<'a> ShrinkSet {
    pub fn new(size: usize) -> Self {
        Self {
            p: size,
            map: (0..size).collect(),
            values: (0..size).collect(),
        }
    }
    pub fn contains(&self, value: usize) -> bool {
        value < self.map.len() && self.map[value] < self.p
    }

    pub fn refill(&mut self) {
        self.p = self.map.len();
    }

    /// Returns an iterator over the values in the set.
    /// Uniqueness is guaranteed; ordering is not.
    pub fn iter(&'a self) -> impl Iterator<Item = &usize> + 'a {
        self.values.iter().take(self.p)
    }

    pub fn is_empty(&self) -> bool {
        self.p == 0
    }

    pub fn remove(&mut self, item: usize) -> usize {
        if self.contains(item) {
            let item_index = self.map[item];
            let last_item = self.values[self.p - 1];
            let last_item_index = self.map[last_item];

            self.values[last_item_index] = item;
            self.values[item_index] = last_item;
            self.map[last_item] = item_index;
            self.map[item] = last_item_index;
            self.p -= 1;
        }

        item
    }
}
