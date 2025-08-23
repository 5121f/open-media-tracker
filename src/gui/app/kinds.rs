// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use derive_more::Display;

#[derive(Clone, Display)]
pub enum ConfirmKind {
    #[display("You actually want to delete media \"{name}\" from the list?")]
    DeleteMedia { name: String, id: usize },
}

#[derive(PartialEq, Eq, Hash)]
pub enum LoadingKind {
    ReadMedia,
}
