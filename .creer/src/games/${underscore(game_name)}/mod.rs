<%include file="functions.noCreer" />\
${shared['rs']['obj_doc'](game, '//! ')}

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
mod ${underscore(obj_key)};
% endfor

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
pub use ${underscore(obj_key)}::${obj_key};
% endfor

#[derive(Debug)]
struct Context {}

impl Context {
    fn try_get_obj<T>(&self, id: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_obj<T>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }
}
