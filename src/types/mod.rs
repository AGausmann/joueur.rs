//! Compound data types designed specifically for Joueur.
//!
//! Reconciling the borrow checker with the current design of the game framework is possible but
//! difficult. It would be nice to use builtin types like slices and `std::collections`, but
//! interior mutability is used heavily due to the many side effects and cross references. The
//! resulting API would be pretty un-ergonomic, so instead we use these types.
//!
//! These structures are reference-counted internally, meaning these act as cheaply-cloneable
//! references that only clone their inner buffers when unique writable access is required. This is
//! a technique known as "Clone-On-Write" in Rust, an analogue to "Copy-On-Write" but renamed after
//! the `Clone` trait.
//!
//! **Note** - These do not behave like game object types, where their state is globally consistent
//! across all references. They instead behave more like primitive data types, where mutations are
//! completely localized and don't mutate the referenced value in other locations. This means that
//! you cannot hold a reference to a game object's list attribute and expect it to update when the
//! game updates; you have to fetch that attribute again from the game object itself.

pub mod list;
pub mod map;
pub mod str;

#[doc(inline)]
pub use self::list::List;
#[doc(inline)]
pub use self::map::Map;
#[doc(inline)]
pub use self::str::Str;
