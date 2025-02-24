use super::Placeholder;

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
impl<T, E> MaybeError<T, E> {
    pub const fn success(value: T) -> Self {
        Self { value, error: None }
    }
}
