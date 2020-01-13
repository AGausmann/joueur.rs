<%include file="functions.noCreer" />\
#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

% if obj_key == 'Game':
/// Holds top-level game state and settings for the current game.
% else:
${shared['rs']['obj_doc'](obj, '/// ')}
% endif
#[derive(Debug, Clone)]
pub struct ${obj_key} {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<${obj_key}Inner>>,
}

#[derive(Debug, Clone)]
struct ${obj_key}Inner {
    ${underscore(obj_key)}: Arc<Mutex<${obj_key}Base>>,
% for parent in shared['rs']['all_parents'](obj):
    ${underscore(parent)}: Arc<Mutex<${underscore(parent)}::${parent}Base>>,
% endfor
}

#[derive(Debug)]
pub(crate) struct ${obj_key}Base {
% for attr_name, attr in obj['attributes'].items():
    pub(crate) ${shared['rs']['sanitize'](underscore(attr_name))}: ${shared['rs']['internal_type'](attr['type'])},
% endfor
}

impl ${obj_key} {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<${obj_key}Inner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: ${obj_key} = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }

% for attr_name, attr, parent in shared['rs']['all_attributes'](obj):

${shared['rs']['attr_doc'](attr, parent, '    /// ')}
    pub fn ${shared['rs']['sanitize'](underscore(attr_name))}(&self) -> ${shared['rs']['return_type'](attr['type'])} {
        self.inner().${underscore(parent or obj_key)}.lock().unwrap().${shared['rs']['sanitize'](underscore(attr_name))}.clone()
    }
% endfor
% for func_name, func, parent in shared['rs']['all_functions'](obj):

${shared['rs']['func_doc'](func, parent, '    /// ')}
    pub fn ${shared['rs']['sanitize'](underscore(func_name))}(
        &self,
% for arg in func['arguments']:
        _${shared['rs']['sanitize'](underscore(arg['name']))}: ${shared['rs']['arg_type'](arg['type'])},
% endfor
    )
% if func['returns']:
        -> ${shared['rs']['return_type'](func['returns']['type'])}
% endif
    {
        unimplemented!()
    }
% endfor

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
