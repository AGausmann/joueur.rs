use crate::error::Error;

use serde_json::{Map, Number, Value};

pub trait DeltaMergeable {
    fn merge_delta(&mut self, delta: Value, cx: &Context) -> Result<(), Error>;
}

pub struct Context {
    delta_list_length: String,
    delta_removed: String,
}

impl Context {
    pub fn delta_list_length(&self) -> &str {
        &self.delta_list_length
    }

    pub fn delta_removed(&self) -> &str {
        &self.delta_removed
    }
}

impl<T> DeltaMergeable for &mut T
where
    T: DeltaMergeable,
{
    fn merge_delta(&mut self, delta: Value, cx: &Context) -> Result<(), Error> {
        T::merge_delta(*self, delta, cx)
    }
}

macro_rules! merge_deserialize {
    ($($t:ty,)*) => {$(
        impl DeltaMergeable for $t {
            fn merge_delta(&mut self, delta: Value, _cx: &Context) -> Result<(), Error> {
                *self = serde_json::from_value(delta)?;
                Ok(())
            }
        }
    )*}
}

merge_deserialize! {
    (),
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    Number,
    String,
}

impl<T> DeltaMergeable for Vec<T>
where
    T: DeltaMergeable + Default,
{
    fn merge_delta(&mut self, delta: Value, cx: &Context) -> Result<(), Error> {
        let mut map: Map<String, Value> = serde_json::from_value(delta)?;
        if let Some(len_value) = map.remove(cx.delta_list_length()) {
            let len = serde_json::from_value(len_value)?;
            self.resize_with(len, Default::default);
        }

        for (key, value) in map {
            let idx: usize = key.parse()?;
            if value == cx.delta_removed() {
                self[idx] = Default::default();
            } else {
                self[idx].merge_delta(value, cx)?;
            }
        }

        Ok(())
    }
}

impl DeltaMergeable for Map<String, Value> {
    fn merge_delta(&mut self, delta: Value, cx: &Context) -> Result<(), Error> {
        let map: Map<String, Value> = serde_json::from_value(delta)?;
        for (key, value) in map {
            if value == cx.delta_removed() {
                self.remove(&key);
            } else {
                self.entry(key)
                    .or_insert(Value::Null)
                    .merge_delta(value, cx)?;
            }
        }
        Ok(())
    }
}

impl DeltaMergeable for Value {
    fn merge_delta(&mut self, delta: Value, cx: &Context) -> Result<(), Error> {
        match self {
            Value::Array(vec) => vec.merge_delta(delta, cx)?,
            Value::Object(map) => map.merge_delta(delta, cx)?,
            this if delta.is_object() => {
                if delta.get(cx.delta_list_length()).is_some() {
                    *this = Value::Array(Vec::new());
                } else {
                    *this = Value::Object(Map::new());
                }
                this.merge_delta(delta, cx)?;
            }
            this => *this = delta,
        }
        Ok(())
    }
}
