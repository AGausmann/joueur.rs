use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::{fmt, ops};

use serde::de::{self, Deserialize, Deserializer, Error, MapAccess, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_derive::{Deserialize, Serialize};
use serde_json as json;
use serde_json::value::RawValue;

use crate::client::base::ObjRef;

thread_local! {
    static DELTA_REMOVED: RefCell<String> = RefCell::new("&RM".to_string());
    static DELTA_LIST_LENGTH: RefCell<String> = RefCell::new("&LEN".to_string());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Delta<'a, T> {
    #[serde(rename = "type", borrow)]
    type_: Borrowable<'a, str>,
    game: GameDelta<'a, T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDelta<'a, T> {
    #[serde(default, borrow)]
    players: CollectionDelta<'a, ObjRef<'a>>,

    #[serde(default = "Default::default", borrow)]
    game_objects: CollectionDelta<'a, T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDelta<'a> {
    #[serde(borrow)]
    id: Borrowable<'a, str>,
}

#[derive(Debug, Clone)]
pub struct CollectionDelta<'a, T> {
    entries: Vec<(Cow<'a, str>, Option<T>)>,
    len: Option<usize>,
}

impl<'a, T> Serialize for CollectionDelta<'a, T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.entries.len()))?;
        for (key, value) in &self.entries {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<'a, 'de: 'a, T> Deserialize<'de> for CollectionDelta<'a, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T> Visitor<'de> for MyVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = CollectionDelta<'de, T>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a map")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                DELTA_REMOVED.with(|delta_removed| {
                    DELTA_LIST_LENGTH.with(|delta_list_length| {
                        let delta_removed = delta_removed.borrow();
                        let delta_list_length = delta_list_length.borrow();

                        let mut entries;
                        if let Some(size) = map.size_hint() {
                            entries = Vec::with_capacity(size);
                        } else {
                            entries = Vec::new();
                        }
                        let mut len = None;

                        while let Some((key, value)) =
                            map.next_entry::<Borrowable<_>, &'de RawValue>()?
                        {
                            if key.0 == *delta_list_length {
                                len = Some(
                                    json::from_str(value.get())
                                        .map_err(|err| A::Error::custom(err))?,
                                );
                            } else if json::from_str::<Borrowable<str>>(value.get())
                                .map(|s| &*s == *delta_removed)
                                .unwrap_or(false)
                            {
                                entries.push((key.0, None));
                            } else {
                                entries.push((
                                    key.0,
                                    Some(
                                        json::from_str(value.get())
                                            .map_err(|err| A::Error::custom(err))?,
                                    ),
                                ));
                            }
                        }

                        Ok(CollectionDelta { entries, len })
                    })
                })
            }
        }
        deserializer.deserialize_map(MyVisitor(PhantomData))
    }
}

impl<'a, T> Default for CollectionDelta<'a, T> {
    fn default() -> Self {
        CollectionDelta {
            entries: Default::default(),
            len: Default::default(),
        }
    }
}

struct Borrowable<'a, T: ?Sized + ToOwned>(Cow<'a, T>);

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
        if f.alternate() {
            write!(f, "{:#?}", self.0)
        } else {
            write!(f, "{:?}", self.0)
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;
    use serde_json::value::RawValue;

    type GameObject<'a> = &'a RawValue;

    #[test]
    fn example_gamelog() {
        let example = r#"[
            {
              "type": "start",
              "game": {
                "players": {
                  "0": {
                    "id": "0"
                  },
                  "1": {
                    "id": "1"
                  },
                  "&LEN": 2
                },
                "gameObjects": {
                  "0": {
                    "id": "0",
                    "gameObjectName": "Player",
                    "logs": {
                      "&LEN": 0
                    },
                    "clientType": "Lua",
                    "lost": false,
                    "name": "Move Lua Player",
                    "otherPlayer": {
                      "id": "1"
                    },
                    "reasonLost": "",
                    "reasonWon": "",
                    "timeRemaining": 60000000000,
                    "won": false
                  },
                  "1": {
                    "id": "1",
                    "gameObjectName": "Player",
                    "logs": {
                      "&LEN": 0
                    },
                    "clientType": "JavaScript",
                    "lost": false,
                    "name": "Move JavaScript Player",
                    "otherPlayer": {
                      "id": "0"
                    },
                    "reasonLost": "",
                    "reasonWon": "",
                    "timeRemaining": 60000000000,
                    "won": false
                  },
                  "2": {
                    "id": "2",
                    "gameObjectName": "Unit",
                    "logs": {
                      "&LEN": 0
                    },
                    "hasMoved": false,
                    "owner": {
                      "id": "0"
                    },
                    "x": 8,
                    "y": 8
                  },
                  "3": {
                    "id": "3",
                    "gameObjectName": "Unit",
                    "logs": {
                      "&LEN": 0
                    },
                    "hasMoved": false,
                    "owner": {
                      "id": "1"
                    },
                    "x": 9,
                    "y": 10
                  }
                },
                "session": "2",
                "name": "Move",
                "currentTurn": 0,
                "maxTurns": 100,
                "currentPlayer": {
                  "id": "0"
                },
                "units": {
                  "0": {
                    "id": "2"
                  },
                  "1": {
                    "id": "3"
                  },
                  "&LEN": 2
                }
              }
            },
            {
              "type": "ran",
              "data": {
                "player": {
                  "id": "0"
                },
                "run": {
                  "functionName": "move",
                  "caller": {
                    "id": "2"
                  },
                  "args": {
                    "y": 9,
                    "x": 9
                  }
                },
                "returned": true
              },
              "game": {
                "gameObjects": {
                  "0": {
                    "timeRemaining": 59998045033
                  },
                  "2": {
                    "x": 9,
                    "y": 9,
                    "hasMoved": true
                  }
                }
              }
            },
            {
              "type": "finished",
              "data": {
                "player": {
                  "id": "0"
                },
                "order": "runTurn",
                "returned": true
              },
              "game": {
                "gameObjects": {
                  "0": {
                    "timeRemaining": 59959439522
                  },
                  "1": {
                    "timeRemaining": 60100000000
                  },
                  "2": {
                    "hasMoved": false
                  }
                },
                "currentTurn": 1,
                "currentPlayer": {
                  "id": "1"
                }
              }
            },
            {
              "type": "ran",
              "data": {
                "player": {
                  "id": "1"
                },
                "run": {
                  "functionName": "move",
                  "caller": {
                    "id": "3"
                  },
                  "args": {
                    "y": 9,
                    "x": 9
                  }
                },
                "returned": true
              },
              "game": {
                "gameObjects": {
                  "0": {
                    "lost": true,
                    "reasonLost": "Got moved on."
                  },
                  "1": {
                    "timeRemaining": 60099785443,
                    "won": true,
                    "reasonWon": "Moved to other unit."
                  },
                  "2": {
                    "x": -1,
                    "y": -1
                  },
                  "3": {
                    "y": 9,
                    "hasMoved": true
                  }
                }
              }
            },
            {
              "type": "over",
              "game": {}
            }
          ]"#;

        let deltas: Vec<Delta<GameObject>> = from_str(example)
            .expect("parsing error");
        eprintln!("{:#?}", deltas);
    }
}
