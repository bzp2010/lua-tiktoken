use anyhow::{anyhow, Ok, Result};
use mlua::prelude::{Lua, LuaResult, LuaValue};
use tiktoken_rs::{cl100k_base, p50k_base, p50k_edit, r50k_base, CoreBPE};

pub fn get_model_list() -> Vec<String> {
    return vec![
        String::from("r50k_base"),
        String::from("p50k_base"),
        String::from("p50k_edit"),
        String::from("cl100k_base"),
    ];
}

pub fn get_model(name: String) -> Result<CoreBPE> {
    match name.as_str() {
        "r50k_base" => Ok(r50k_base().unwrap()),
        "p50k_base" => Ok(p50k_base().unwrap()),
        "p50k_edit" => Ok(p50k_edit().unwrap()),
        "cl100k_base" => Ok(cl100k_base().unwrap()),
        _ => Err(anyhow!("Missing model: {}", name)),
    }
}

// Lua land API
pub fn models<'lua>(lua: &'lua Lua, _: ()) -> LuaResult<LuaValue<'lua>> {
    LuaResult::Ok(LuaValue::Table(lua.create_sequence_from(get_model_list())?))
}
