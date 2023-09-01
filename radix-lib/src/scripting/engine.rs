use crate::scripting::log::{debug, error, info, trace, warn};
use mlua::{Function, Lua};
use winit_input_helper::WinitInputHelper;

use super::input::{key_held, key_pressed, key_released};

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

        self.lua.set_app_data(WinitInputHelper::new());

        let key_held = self.lua.create_function(key_held).unwrap();
        let key_pressed = self.lua.create_function(key_pressed).unwrap();
        let key_released = self.lua.create_function(key_released).unwrap();

        self.lua.globals().set("key_held", key_held).unwrap();
        self.lua.globals().set("key_pressed", key_pressed).unwrap();
        self.lua
            .globals()
            .set("key_released", key_released)
            .unwrap();
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

    pub fn update_input(&self, event: &winit::event::Event<()>) -> bool {
        let mut input = self.lua.app_data_mut::<WinitInputHelper>().unwrap();
        input.update(event)
    }
}
