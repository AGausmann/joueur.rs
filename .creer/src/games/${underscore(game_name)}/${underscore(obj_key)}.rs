<%include file="functions.noCreer" />\
#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

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
% if obj_key == 'Game':
                panic!("game is unresolved?");
% else:
                let obj: ${obj_key} = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
% endif
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
        ${shared['rs']['sanitize'](underscore(arg['name']))}: ${shared['rs']['arg_type'](arg['type'])},
% endfor
    )
% if func['returns']:
        -> Result<${shared['rs']['return_type'](func['returns']['type'])}, Error>
% else:
        -> Result<(), Error>
% endif
    {
        struct Args<'a> {
% for arg in func['arguments']:
            ${shared['rs']['sanitize'](underscore(arg['name']))}: ${shared['rs']['arg_type'](arg['type'], 'a')},
% endfor
            _a: PhantomData< &'a () >,
        }
        let args = Args {
% for arg in func['arguments']:
            ${shared['rs']['sanitize'](underscore(arg['name']))},
% endfor
            _a: PhantomData,
        };
        self.context().run(&self.id, "${func_name}", args)
    }
% endfor
% if obj_key != 'Game':

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.context().get_obj(&self.id)
    }

    pub(crate) fn try_upcast<T: Object>(&self) -> Option<T> {
        match TypeId::of::<T>() {
            x if x == TypeId::of::<${obj_key}>() => Some(T::shallow(self.context.clone(), self.id.clone())),
% for parent in shared['rs']['all_parents'](obj):
            x if x == TypeId::of::<${parent}>() => Some(T::shallow(self.context.clone(), self.id.clone())),
% endfor
            _ => None,
        }
    }
% endif
}
% if obj_key != 'Game':

impl ObjectInner for ${obj_key} {
    fn shallow(context: Weak<Context>, id: Str) -> ${obj_key} {
        ${obj_key} {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for ${obj_key} {}
% endif
