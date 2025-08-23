// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

pub trait Placeholder {
    fn placeholder() -> Self;
}

impl<T> Placeholder for T
where
    T: Default,
{
    fn placeholder() -> Self {
        Self::default()
    }
}
