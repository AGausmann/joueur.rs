<%include file="functions.noCreer" />\
${shared['rs']['obj_doc'](game, '//! ')}

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
mod ${underscore(obj_key)};
% endfor

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
pub use ${underscore(obj_key)}::${obj_key};
% endfor

use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex, Weak};

use crate::error::Error;
use crate::types::*;

#[doc(hidden)]
#[derive(Debug)]
pub struct Context {
    game_objects: Map<Str, Box<dyn Any>>,
}

impl Context {
    fn try_get_obj<T: Object>(&self, id: &str) -> Option<T> {
        self.game_objects.get(id)
            .and_then(|obj| match obj.type_id() {
% for obj_key in sorted(list(game_objs.keys())):
                x if x == TypeId::of::<${obj_key}>() => obj
                    .downcast_ref::<${obj_key}>()
                    .and_then(|base| T::from_bases(base.to_bases())),
% endfor
                _ => panic!("unknown game object type"),
            })
    }

    fn get_obj<T: Object>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }

    fn run<T, R>(&self, _caller: &str, _function_name: &str, _args: T) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner  {}

mod inner {
    use super::*;

    pub trait ObjectInner: Any + Sized {
        fn to_bases(&self) -> Bases;

        fn from_bases(bases: Bases) -> Option<Self>;
    }
}

use inner::ObjectInner;

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Bases {
    context: Option<Weak<Context>>,
    id: Option<Str>,
% for obj_key in sorted(list(game_objs.keys())):
    ${underscore(obj_key)}: Option<Arc<Mutex<${underscore(obj_key)}::${obj_key}Base>>>,
% endfor
}
