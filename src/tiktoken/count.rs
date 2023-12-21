use mlua::prelude::{Lua, LuaResult, LuaValue};

use crate::tiktoken::model::get_model;

pub fn count<'lua>(
    lua: &'lua Lua,
    (model, message): (String, String),
) -> LuaResult<(LuaValue<'lua>, LuaValue<'lua>)> {
    let bpe = match get_model(model) {
        Err(err) => {
            return Ok((
                LuaValue::Nil,
                LuaValue::String(lua.create_string(&err.to_string())?),
            ))
        }
        Ok(bpe) => bpe,
    };

    let tokens = bpe.encode_with_special_tokens(message.as_str());

    Ok((
      LuaValue::Integer(tokens.len() as i64),
      LuaValue::Nil,
    ))
}
