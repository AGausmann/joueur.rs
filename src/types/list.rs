//! Associated types for `List`.

use std::borrow::{Borrow, BorrowMut};
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

impl<T> List<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut [T]
    where
        T: Clone,
    {
        &mut *Arc::make_mut(&mut self.inner)
    }
}

impl<T> ops::Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> ops::DerefMut for List<T>
where
    T: Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> AsRef<[T]> for List<T> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> AsMut<[T]> for List<T>
where
    T: Clone,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T> Borrow<[T]> for List<T> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> BorrowMut<[T]> for List<T>
where
    T: Clone,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
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
