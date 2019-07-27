use std::rc::Rc;

pub struct List<T> {
    _inner: Rc<Vec<T>>,
}
