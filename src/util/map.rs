use std::collections::HashMap;
use std::rc::Rc;

pub struct Map<K, V> {
    _inner: Rc<HashMap<K, V>>,
}
