/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{collections::HashSet, hash::Hash};

pub struct LoadingQueue<T>
where
    T: LoadingKind,
{
    kinds: HashSet<T>,
}

impl<T> LoadingQueue<T>
where
    T: LoadingKind,
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

pub trait LoadingKind: PartialEq + Eq + Hash {}
