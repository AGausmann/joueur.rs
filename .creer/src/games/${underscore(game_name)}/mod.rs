<%include file="functions.noCreer" />\
${shared['rs']['obj_doc'](game, '//! ')}

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
mod ${underscore(obj_key)};
% endfor

% for obj_key in sorted(list(game_objs.keys()) + ['Game', 'AI']):
pub use ${underscore(obj_key)}::${obj_key};
% endfor
