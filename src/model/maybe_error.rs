// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

use crate::model::Placeholder;

#[derive(Debug, Clone)]
pub struct MaybeError<T, E> {
    pub value: T,
    pub error: Option<E>,
}

impl<T, E> MaybeError<T, E>
where
    T: Placeholder,
{
    pub fn error(err: E) -> Self {
        Self {
            value: T::placeholder(),
            error: Some(err),
        }
    }
}
