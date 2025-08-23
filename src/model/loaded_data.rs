// SPDX-License-Identifier: MPL-2.0 OR GPL-2.0-or-later

#[derive(Debug, Clone)]
pub enum LoadedData<T, E> {
    Loading,
    Some(T),
    Err(E),
}

impl<T, E> LoadedData<T, E> {
    pub const fn as_opt_res(&self) -> Option<Result<&T, &E>> {
        match self {
            Self::Loading => None,
            Self::Some(value) => Some(Ok(value)),
            Self::Err(err) => Some(Err(err)),
        }
    }

    pub const fn as_option(&self) -> Option<&T> {
        if let Self::Some(value) = self {
            return Some(value);
        }
        None
    }

    pub const fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }
}

impl<T, E> From<Result<T, E>> for LoadedData<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::Some(value),
            Err(err) => Self::Err(err),
        }
    }
}
