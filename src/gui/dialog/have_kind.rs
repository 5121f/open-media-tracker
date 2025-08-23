// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

pub trait HaveKind {
    type Kind;

    fn kind(&self) -> &Self::Kind;
}
