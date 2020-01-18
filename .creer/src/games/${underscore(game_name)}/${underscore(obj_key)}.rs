<%include file="functions.noCreer" />\
#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

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
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl ${obj_key} {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }
% for attr_name, attr, parent in shared['rs']['all_attributes'](obj):

${shared['rs']['attr_doc'](attr, parent, '    /// ')}
    pub fn ${shared['rs']['sanitize'](underscore(attr_name))}(&self) -> ${shared['rs']['return_type'](attr['type'])} {
% if obj_key == 'Game':
        unimplemented!()
% else:
        self.inner.lock().unwrap().as_${underscore(parent or obj_key)}()
            .${shared['rs']['sanitize'](underscore(attr_name))}.clone()
% endif
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
        self.with_context(|cx| cx.run(&self.id(), "${func_name}", args))
    }
% endfor

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        T::from_game_object(&self.inner, &self.context)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.try_cast().unwrap()
    }
}
% if obj_key != 'Game':

impl inner::ObjectInner for ${obj_key} {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_${underscore(obj_key)}().is_some() {
            Some(${obj_key} {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for ${obj_key} {}
% endif
