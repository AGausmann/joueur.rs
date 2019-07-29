<%include file="functions.noCreer" />\
#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

${shared['rs']['obj_doc'](ai, '/// ')}
pub struct AI {
}

impl AI {
% for func_name, func in ai['functions'].items():

${shared['rs']['func_doc'](func, None, '    /// ')}
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
}