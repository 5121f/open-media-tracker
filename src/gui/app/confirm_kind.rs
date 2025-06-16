/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use derive_more::Display;

#[derive(Clone, Display)]
pub enum ConfirmKind {
    #[display("You actually want to delete media \"{name}\" from the list?")]
    DeleteMedia { name: String, id: usize },
}
