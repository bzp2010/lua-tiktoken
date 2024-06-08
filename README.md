# OpenAI tiktoken for Lua

The `tiktoken-rs` is a rust library for tokenizing text with OpenAI models using tiktoken.

This project provides `tiktoken-rs` bindings on the Lua, which enables out-of-the-box text tokenization for AI-related projects such as Apache APISIX.

## Installation

The preferred way to install this library is to use Luarocks:

```shell
luarocks install https://github.com/bzp2010/lua-tiktoken/raw/main/rockspec/lua-tiktoken-main-0.rockspec

```

## Synopsis

```lua
local tiktoken = require("tiktoken")
local models = tiktoken.models()   -- {"r50k_base", "p50k_base", "p50k_edit", "cl100k_base"}
local count = tiktoken.count("cl100k_base", "Here is a message")
```

## Modules

### tiktoken

To load this module:

```lua
local tiktoken = require("tiktoken")
```

#### models

**syntax:** *models = tiktoken.models()*

It returns a string table containing all the tokenizer models.

#### count

**syntax:** *count = tiktoken.count(model_name, text)*

`model_name` is the tokenizer model to be used, and it should be one of the tables returned by `models()`.

`message` is the text to be tokenized.

It returns the number of tokens.
