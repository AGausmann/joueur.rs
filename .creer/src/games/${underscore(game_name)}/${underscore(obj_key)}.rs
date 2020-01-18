<%include file="functions.noCreer" />\
#![allow(dead_code, unused_imports)]

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
}

impl ${obj_key} {
% for attr_name, attr, parent in shared['rs']['all_attributes'](obj):

${shared['rs']['attr_doc'](attr, parent, '    /// ')}
    pub fn ${shared['rs']['sanitize'](underscore(attr_name))}(&self) -> ${shared['rs']['return_type'](attr['type'])} {
        unimplemented!()
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
        -> Result<${shared['rs']['return_type'](func['returns']['type'])}, Error>
% else:
        -> Result<(), Error>
% endif
    {
        unimplemented!()
    }
% endfor
}
