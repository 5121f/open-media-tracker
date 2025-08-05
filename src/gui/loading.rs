/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::hash::Hash;
use std::ops::Deref;

use crate::gui::Dialog;
use crate::gui::page::LoadingPage;

pub struct LoadingDialog<K: PartialEq + Eq + Hash>(Dialog<LoadingPage<K>>);

impl<K> LoadingDialog<K>
where
    K: PartialEq + Eq + Hash,
{
    pub const fn closed() -> Self {
        Self(Dialog::closed())
    }

    pub fn insert(&mut self, kind: K) {
        let dialog = self.0.get_or_insert(LoadingPage::new());
        dialog.add(kind);
    }

    pub fn complete(&mut self, kind: &K) {
        let Some(queue) = &mut *self.0 else {
            return;
        };
        queue.complete(kind);
        if queue.completed() {
            self.0.close();
        }
    }
}

impl<K> Deref for LoadingDialog<K>
where
    K: PartialEq + Eq + Hash,
{
    type Target = Dialog<LoadingPage<K>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> Default for LoadingDialog<K>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self(Dialog::default())
    }
}
