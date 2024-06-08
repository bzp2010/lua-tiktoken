use mlua::prelude::*;

mod tiktoken;

#[mlua::lua_module]
fn tiktoken(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    
    // Get models list
    exports.set("models", lua.create_function(tiktoken::model::models)?)?;

    // Count tokens
    exports.set("count", lua.create_function(tiktoken::count::count)?)?;

    Ok(exports)
}
