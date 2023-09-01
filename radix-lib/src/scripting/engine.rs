use crate::scripting::log::{debug, error, info, trace, warn};
use mlua::Lua;

pub struct Engine {
    lua: Lua,
    script_path: String,
}

impl Engine {
    pub fn new(script_path: &str) -> Self {
        Self {
            lua: Lua::new(),
            script_path: script_path.to_string(),
        }
    }

    pub fn test(&self) {
        let script = std::fs::read_to_string(&self.script_path).unwrap();

        let trace = self.lua.create_function(trace).unwrap();
        let debug = self.lua.create_function(debug).unwrap();
        let info = self.lua.create_function(info).unwrap();
        let warn = self.lua.create_function(warn).unwrap();
        let error = self.lua.create_function(error).unwrap();

        self.lua.globals().set("trace", trace).unwrap();
        self.lua.globals().set("debug", debug).unwrap();
        self.lua.globals().set("info", info).unwrap();
        self.lua.globals().set("warn", warn).unwrap();
        self.lua.globals().set("error", error).unwrap();

        self.lua.load(&script).exec().unwrap();
    }
}
