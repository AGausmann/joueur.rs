//! Associated types for `List`.

use std::iter::FromIterator;
use std::ops;
use std::sync::Arc;

/// An internally reference-counted list structure.
///
/// This type also dereferences to [`[T]`][slice], meaning you can use this anywhere where a slice
/// is accepted. This includes calling slice methods such as `get`, `sort`, and `iter`.
///
/// [slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct List<T> {
    inner: Arc<Vec<T>>,
}

impl<T> ops::Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ops::DerefMut for List<T>
where
    T: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *Arc::make_mut(&mut self.inner)
    }
}

impl<T> From<&[T]> for List<T>
where
    T: Clone,
{
    fn from(slice: &[T]) -> List<T> {
        slice.to_vec().into()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(vec: Vec<T>) -> List<T> {
        List {
            inner: Arc::new(vec),
        }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I>(iter: I) -> List<T>
    where
        I: IntoIterator<Item = T>,
    {
        iter.into_iter().collect::<Vec<T>>().into()
    }
}

impl<T> PartialEq<[T]> for List<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        &*self.inner == &other
    }
}

impl<T> PartialEq<Vec<T>> for List<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<T>) -> bool {
        &*self.inner == other
    }
}
