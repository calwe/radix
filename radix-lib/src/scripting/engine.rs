use crate::scripting::log::{debug, error, info, trace, warn};
use mlua::{Function, Lua};

pub struct Engine {
    lua: Lua,
    script: String,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            script: String::new(),
        }
    }

    pub fn load_globals(&self) {
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
    }

    pub fn load_script(&mut self, path: &str) {
        self.script = std::fs::read_to_string(path).unwrap();
        self.lua.load(&self.script).exec().unwrap();
    }

    pub fn start(&self) {
        let start: Function = self.lua.globals().get("start").unwrap();
        start.call::<(), ()>(()).unwrap();
    }

    pub fn update(&self) {
        let update: Function = self.lua.globals().get("update").unwrap();
        update.call::<(), ()>(()).unwrap();
    }
}
