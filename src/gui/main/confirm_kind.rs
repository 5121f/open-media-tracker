/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::fmt::Display;

#[derive(Clone)]
pub enum ConfirmKind {
    DeleteMedia { name: String, id: usize },
}

impl Display for ConfirmKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DeleteMedia { name, .. } => {
                write!(
                    f,
                    "You actually want to delete media \"{name}\" from the list?",
                )
            }
        }
    }
}
