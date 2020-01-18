<%include file="functions.noCreer" />\
#![allow(unused_imports, dead_code)]

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct Context {
    game: GameBase,
}

impl Context {
    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GameObject {
% for obj in game_objs.keys():
    ${obj}(${obj}Inner),
% endfor
}

impl GameObject {
    pub(crate) fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub(crate) fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }
% for obj_key in game_objs.keys():

    pub(crate) fn try_as_${underscore(obj_key)}(&self) -> Option< &${obj_key}Base > {
        match self {
% for sub_key, sub_obj in game_objs.items():
% if sub_key == obj_key or obj_key in shared['rs']['all_parents'](sub_obj):
            GameObject::${sub_key}(obj) => Some(&obj.${underscore(obj_key)}),
% else:
            GameObject::${sub_key}(_obj) => None,
% endif
% endfor
        }
    }

    pub(crate) fn as_${underscore(obj_key)}(&self) -> &${obj_key}Base {
        self.try_as_${underscore(obj_key)}().expect("unreachable: unable to cast to ${obj_key}")
    }
% endfor
}
% for obj_key, obj in game_objs.items():

#[derive(Debug, Clone)]
pub(crate) struct ${obj_key}Inner {
    pub(crate) ${underscore(obj_key)}: ${obj_key}Base,
% for parent in shared['rs']['all_parents'](obj):
    pub(crate) ${underscore(parent)}: ${parent}Base,
% endfor
}
% endfor
% for obj_key, obj in list(game_objs.items()) + [('Game', game)]:

#[derive(Debug, Clone)]
pub(crate) struct ${obj_key}Base {
% for attr_name, attr in obj['attributes'].items():
    pub(crate) ${shared['rs']['sanitize'](underscore(attr_name))}: ${shared['rs']['internal_type'](attr['type'])},
% endfor
}
% endfor
