use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::{fmt, ops};

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};

pub struct Borrowable<'a, T: ?Sized + ToOwned>(pub Cow<'a, T>);

impl<'a> Serialize for Borrowable<'a, str> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(ser)
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Borrowable<'a, str> {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = Borrowable<'de, str>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "an owned or borrowed string")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Borrowable(Cow::Borrowed(v)))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Borrowable(Cow::Owned(v.to_string())))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Borrowable(Cow::Owned(v)))
            }
        }

        de.deserialize_str(MyVisitor)
    }
}

impl<'a, T: ?Sized + ToOwned> fmt::Debug for Borrowable<'a, T>
where
    T: fmt::Debug,
    T::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl<'a, T: ?Sized + ToOwned> Clone for Borrowable<'a, T> {
    fn clone(&self) -> Self {
        Borrowable(self.0.clone())
    }
}

impl<'a, 'b, B, C> PartialEq<Borrowable<'b, C>> for Borrowable<'a, B>
where
    B: PartialEq<C> + ToOwned + ?Sized,
    C: ToOwned + ?Sized,
{
    fn eq(&self, other: &Borrowable<'b, C>) -> bool {
        self.0 == other.0
    }
}

impl<'a, T> Eq for Borrowable<'a, T> where T: Eq + ToOwned + ?Sized {}

impl<'a, T> Hash for Borrowable<'a, T>
where
    T: Hash + ToOwned + ?Sized,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.hash(state);
    }
}

impl<'a, T> ops::Deref for Borrowable<'a, T>
where
    T: ToOwned + ?Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
