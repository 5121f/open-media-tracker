// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::collections::HashSet;
use std::hash::Hash;

pub struct LoadingQueue<T>
where
    T: PartialEq + Eq + Hash,
{
    kinds: HashSet<T>,
}

impl<T> LoadingQueue<T>
where
    T: PartialEq + Eq + Hash,
{
    pub fn new() -> Self {
        let kinds = HashSet::new();
        Self { kinds }
    }

    pub fn add(&mut self, kind: T) {
        self.kinds.insert(kind);
    }

    pub fn complete(&mut self, kind: &T) {
        self.kinds.remove(kind);
    }

    pub fn completed(&self) -> bool {
        self.kinds.is_empty()
    }
}
