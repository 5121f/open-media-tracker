/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::gui::{screen::LoadingScreen, Dialog};

pub struct LoadingDialog<K>(Dialog<LoadingScreen<K>>);

impl<K> LoadingDialog<K>
where
    K: PartialEq + Eq + Hash,
{
    pub fn closed() -> Self {
        Self(Dialog::closed())
    }

    pub fn insert(&mut self, kind: K) {
        let dialog = self.0.get_or_insert(LoadingScreen::new());
        dialog.insert(kind);
    }

    pub fn complete(&mut self, kind: K) {
        let Some(screen) = self.0.deref_mut() else {
            return;
        };
        screen.complete(kind);
        if screen.all_complete() {
            self.0.close();
        }
    }
}

impl<K> Deref for LoadingDialog<K> {
    type Target = Dialog<LoadingScreen<K>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> Default for LoadingDialog<K> {
    fn default() -> Self {
        Self(Default::default())
    }
}
