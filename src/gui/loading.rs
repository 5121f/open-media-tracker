// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use std::hash::Hash;

use derive_more::Deref;

use crate::gui::Dialog;
use crate::gui::page::LoadingPage;

#[derive(Deref, Default)]
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
