//! Associated types for `Str`.

use std::borrow::{Borrow, BorrowMut};
use std::iter::FromIterator;
use std::ops;
use std::sync::Arc;

/// An internally reference-counted Unicode string.
///
/// This type also dereferences to [`str`][str], meaning you can use it anywhere where string
/// slices are accepted. This includes calling methods on string slices such as `get`, `chars`,
/// and `trim`.
///
/// [str]: https://doc.rust-lang.org/stable/std/primitive.str.html
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Str {
    inner: Arc<String>,
}

impl Str {
    pub fn as_slice(&self) -> &str {
        &*self.inner
    }

    pub fn as_mut_slice(&mut self) -> &mut str {
        &mut *Arc::make_mut(&mut self.inner)
    }
}

impl ops::Deref for Str {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl ops::DerefMut for Str {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl AsRef<str> for Str {
    fn as_ref(&self) -> &str {
        self.as_slice()
    }
}

impl AsMut<str> for Str {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_slice()
    }
}

impl Borrow<str> for Str {
    fn borrow(&self) -> &str {
        self.as_slice()
    }
}

impl BorrowMut<str> for Str {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_slice()
    }
}

impl From<&str> for Str {
    fn from(s: &str) -> Str {
        s.to_string().into()
    }
}

impl From<String> for Str {
    fn from(s: String) -> Str {
        Str {
            inner: Arc::new(s),
        }
    }
}

impl FromIterator<char> for Str {
    fn from_iter<I>(iter: I) -> Str
    where
        I: IntoIterator<Item = char>,
    {
        iter.into_iter().collect::<String>().into()
    }
}

impl Default for Str {
    fn default() -> Self {
        "".into()
    }
}

impl PartialEq<str> for Str {
    fn eq(&self, other: &str) -> bool {
        &*self.inner == other
    }
}

impl PartialEq<String> for Str {
    fn eq(&self, other: &String) -> bool {
        &*self.inner == other
    }
}
