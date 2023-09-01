use log::trace;
use mlua::{Lua, Result, UserData};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub fn key_held(lua: &Lua, key: String) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.key_held(string_to_vkeycode(key).unwrap()))
}

pub fn key_pressed(lua: &Lua, key: String) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.key_pressed(string_to_vkeycode(key).unwrap()))
}

pub fn key_released(lua: &Lua, key: String) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.key_released(string_to_vkeycode(key).unwrap()))
}

pub fn string_to_vkeycode(key: String) -> std::result::Result<VirtualKeyCode, ()> {
    match key.as_str() {
        "A" => Ok(VirtualKeyCode::A),
        _ => Err(()),
    }
}
