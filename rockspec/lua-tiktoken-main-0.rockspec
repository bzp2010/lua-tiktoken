package = "lua-tiktoken"
version = "main-0"

source = {
    url = "git://github.com/bzp2010/lua-tiktoken",
    tag = "main",
}

description = {
    summary = "Lua extension for tiktoken integration",
    detailed = [[
        The Lua module written in Rust that provides tiktoken-rs integration.
    ]],
    homepage = "https://github.com/bzp2010/lua-tiktoken",
    license = "Apache License 2.0"
}

dependencies = {
    "lua >= 5.1",
}

build = {
   type = "make",
   build_variables = {
      CFLAGS="$(CFLAGS)",
      LIBFLAG="$(LIBFLAG)",
      LUA_LIBDIR="$(LUA_LIBDIR)",
      LUA_BINDIR="$(LUA_BINDIR)",
      LUA_INCDIR="$(LUA_INCDIR)",
      LUA="$(LUA)",
   },
   install_variables = {
      INST_PREFIX="$(PREFIX)",
      INST_BINDIR="$(BINDIR)",
      INST_LIBDIR="$(LIBDIR)",
      INST_LUADIR="$(LUADIR)",
      INST_CONFDIR="$(CONFDIR)",
   },
}
