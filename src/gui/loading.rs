/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::ops::{Deref, DerefMut};

use crate::{
    gui::{screen::LoadingScrn, Dialog},
    model::LoadingKind,
};

pub struct LoadingDialog<K: LoadingKind>(Dialog<LoadingScrn<K>>);

impl<K> LoadingDialog<K>
where
    K: LoadingKind,
{
    pub const fn closed() -> Self {
        Self(Dialog::closed())
    }

    pub fn insert(&mut self, kind: K) {
        let dialog = self.0.get_or_insert(LoadingScrn::new());
        dialog.add(kind);
    }

    pub fn complete(&mut self, kind: K) {
        let Some(screen) = self.0.deref_mut() else {
            return;
        };
        screen.complete(kind);
        if screen.completed() {
            self.0.close();
        }
    }
}

impl<K> Deref for LoadingDialog<K>
where
    K: LoadingKind,
{
    type Target = Dialog<LoadingScrn<K>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> Default for LoadingDialog<K>
where
    K: LoadingKind,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
