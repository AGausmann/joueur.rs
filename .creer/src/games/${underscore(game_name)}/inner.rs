<%include file="functions.noCreer" />\
#![allow(unused_imports, dead_code)]

use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Context {
    self_ref: Option<Weak<Mutex<Context>>>,
    game: Game,
}

impl Context {
    pub(crate) fn get_ref(&self) -> Weak<Mutex<Context>> {
        self.self_ref.clone().unwrap()
    }

    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner {
}

pub trait ObjectInner: Sized {
    fn from_game_object(game_obj: &Arc<Mutex<AnyGameObject>>, context: &Weak<Mutex<Context>>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum AnyGameObject {
% for obj in game_objs.keys():
    ${obj}(${obj}Inner),
% endfor
}

impl AnyGameObject {
    pub fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }
% for obj_key in game_objs.keys():

    pub fn try_as_${underscore(obj_key)}(&self) -> Option< &${obj_key}Base > {
        match self {
% for sub_key, sub_obj in game_objs.items():
% if sub_key == obj_key or obj_key in shared['rs']['all_parents'](sub_obj):
            AnyGameObject::${sub_key}(obj) => Some(&obj.${underscore(obj_key)}),
% else:
            AnyGameObject::${sub_key}(_obj) => None,
% endif
% endfor
        }
    }

    pub fn as_${underscore(obj_key)}(&self) -> &${obj_key}Base {
        self.try_as_${underscore(obj_key)}().expect("unreachable: unable to cast to ${obj_key}")
    }
% endfor
}
% for obj_key, obj in game_objs.items():

#[derive(Debug, Clone)]
pub struct ${obj_key}Inner {
    pub ${underscore(obj_key)}: ${obj_key}Base,
% for parent in shared['rs']['all_parents'](obj):
    pub ${underscore(parent)}: ${parent}Base,
% endfor
}
% endfor
% for obj_key, obj in list(game_objs.items()) + [('Game', game)]:

#[derive(Debug, Clone)]
pub struct ${obj_key}Base {
% for attr_name, attr in obj['attributes'].items():
    pub ${shared['rs']['sanitize'](underscore(attr_name))}: ${shared['rs']['internal_type'](attr['type'])},
% endfor
}
% endfor
