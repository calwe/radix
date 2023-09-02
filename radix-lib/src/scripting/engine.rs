use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{
    player::Player,
    scene::Scene,
    scripting::log::{debug, error, info, trace, warn},
};
use mlua::{Function, Lua};
use winit_input_helper::WinitInputHelper;

use super::input::*;

pub struct Engine {
    lua: Lua,
    script: String,
    scene: Option<Rc<RefCell<Scene>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            lua: Lua::new(),
            script: String::new(),
            scene: None,
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
        let held_shift = self.lua.create_function(held_shift).unwrap();
        let held_control = self.lua.create_function(held_control).unwrap();
        let held_alt = self.lua.create_function(held_alt).unwrap();
        let mouse = self.lua.create_function(mouse).unwrap();
        let mouse_x = self.lua.create_function(mouse_x).unwrap();
        let mouse_y = self.lua.create_function(mouse_y).unwrap();
        let mouse_held = self.lua.create_function(mouse_held).unwrap();
        let mouse_pressed = self.lua.create_function(mouse_pressed).unwrap();
        let mouse_released = self.lua.create_function(mouse_released).unwrap();

        let input = self.lua.create_table().unwrap();
        input.set("key_held", key_held).unwrap();
        input.set("key_pressed", key_pressed).unwrap();
        input.set("key_released", key_released).unwrap();
        input.set("held_shift", held_shift).unwrap();
        input.set("held_control", held_control).unwrap();
        input.set("held_alt", held_alt).unwrap();
        input.set("mouse", mouse).unwrap();
        input.set("mouse_x", mouse_x).unwrap();
        input.set("mouse_y", mouse_y).unwrap();
        input.set("mouse_held", mouse_held).unwrap();
        input.set("mouse_pressed", mouse_pressed).unwrap();
        input.set("mouse_released", mouse_released).unwrap();

        input.set("MOUSE_LEFT", 0).unwrap();
        input.set("MOUSE_RIGHT", 1).unwrap();
        input.set("MOUSE_MIDDLE", 2).unwrap();

        self.lua.globals().set("input", input).unwrap();

        let scene = self.scene.as_ref().unwrap().borrow();
        self.lua.globals().set("player", scene.player()).unwrap();
    }

    pub fn load_script(&mut self, path: &str) {
        self.script = std::fs::read_to_string(path).unwrap();
        self.lua.load(&self.script).exec().unwrap();
    }

    pub fn set_scene(&mut self, scene: Rc<RefCell<Scene>>) {
        self.scene = Some(scene);
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
