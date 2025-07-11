/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[derive(Debug, Clone)]
pub enum LoadedData<T, E> {
    Loading,
    Some(T),
    Err(E),
}

impl<T, E> From<Result<T, E>> for LoadedData<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::Some(value),
            Err(err) => Self::Err(err),
        }
    }
}
