<%include file="functions.noCreer" />\
${shared['rs']['obj_doc'](game, '//! ')}

mod inner;

% for obj_key in sorted(game_objs.keys()):
mod ${underscore(obj_key)};
% endfor
mod game;

% for obj_key in sorted(game_objs.keys()):
pub use ${underscore(obj_key)}::${obj_key};
% endfor
pub use game::Game;

pub use inner::Object;
