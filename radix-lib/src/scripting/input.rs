use mlua::{Lua, Result, Table};
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// TODO: missing functions? mouse_diff (will need to be custom)?

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

pub fn held_shift(lua: &Lua, _: ()) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.held_shift())
}

pub fn held_control(lua: &Lua, _: ()) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.held_control())
}

pub fn held_alt(lua: &Lua, _: ()) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.held_alt())
}

pub fn mouse(lua: &Lua, _: ()) -> Result<Table> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    let table = lua.create_table()?;
    let mouse = input.mouse().unwrap_or((0.0, 0.0));
    table.set("x", mouse.0)?;
    table.set("y", mouse.1)?;
    Ok(table)
}

pub fn mouse_x(lua: &Lua, _: ()) -> Result<f32> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.mouse().unwrap_or((0.0, 0.0)).0)
}

pub fn mouse_y(lua: &Lua, _: ()) -> Result<f32> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.mouse().unwrap_or((0.0, 0.0)).1)
}

pub fn mouse_held(lua: &Lua, button: usize) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.mouse_held(button))
}

pub fn mouse_pressed(lua: &Lua, button: usize) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.mouse_pressed(button))
}

pub fn mouse_released(lua: &Lua, button: usize) -> Result<bool> {
    let input = lua.app_data_ref::<WinitInputHelper>().unwrap();
    Ok(input.mouse_released(button))
}

pub fn string_to_vkeycode(key: String) -> std::result::Result<VirtualKeyCode, ()> {
    // TODO: go through this list - some may have aliases / be missing
    match key.to_ascii_lowercase().as_str() {
        "a" => Ok(VirtualKeyCode::A),
        "b" => Ok(VirtualKeyCode::B),
        "c" => Ok(VirtualKeyCode::C),
        "d" => Ok(VirtualKeyCode::D),
        "e" => Ok(VirtualKeyCode::E),
        "f" => Ok(VirtualKeyCode::F),
        "g" => Ok(VirtualKeyCode::G),
        "h" => Ok(VirtualKeyCode::H),
        "i" => Ok(VirtualKeyCode::I),
        "j" => Ok(VirtualKeyCode::J),
        "k" => Ok(VirtualKeyCode::K),
        "l" => Ok(VirtualKeyCode::L),
        "m" => Ok(VirtualKeyCode::M),
        "n" => Ok(VirtualKeyCode::N),
        "o" => Ok(VirtualKeyCode::O),
        "p" => Ok(VirtualKeyCode::P),
        "q" => Ok(VirtualKeyCode::Q),
        "r" => Ok(VirtualKeyCode::R),
        "s" => Ok(VirtualKeyCode::S),
        "t" => Ok(VirtualKeyCode::T),
        "u" => Ok(VirtualKeyCode::U),
        "v" => Ok(VirtualKeyCode::V),
        "w" => Ok(VirtualKeyCode::W),
        "x" => Ok(VirtualKeyCode::X),
        "y" => Ok(VirtualKeyCode::Y),
        "z" => Ok(VirtualKeyCode::Z),
        "0" => Ok(VirtualKeyCode::Key0),
        "1" => Ok(VirtualKeyCode::Key1),
        "2" => Ok(VirtualKeyCode::Key2),
        "3" => Ok(VirtualKeyCode::Key3),
        "4" => Ok(VirtualKeyCode::Key4),
        "5" => Ok(VirtualKeyCode::Key5),
        "6" => Ok(VirtualKeyCode::Key6),
        "7" => Ok(VirtualKeyCode::Key7),
        "8" => Ok(VirtualKeyCode::Key8),
        "9" => Ok(VirtualKeyCode::Key9),
        "escape" | "esc" => Ok(VirtualKeyCode::Escape),
        "f1" => Ok(VirtualKeyCode::F1),
        "f2" => Ok(VirtualKeyCode::F2),
        "f3" => Ok(VirtualKeyCode::F3),
        "f4" => Ok(VirtualKeyCode::F4),
        "f5" => Ok(VirtualKeyCode::F5),
        "f6" => Ok(VirtualKeyCode::F6),
        "f7" => Ok(VirtualKeyCode::F7),
        "f8" => Ok(VirtualKeyCode::F8),
        "f9" => Ok(VirtualKeyCode::F9),
        "f10" => Ok(VirtualKeyCode::F10),
        "f11" => Ok(VirtualKeyCode::F11),
        "f12" => Ok(VirtualKeyCode::F12),
        "f13" => Ok(VirtualKeyCode::F13),
        "f14" => Ok(VirtualKeyCode::F14),
        "f15" => Ok(VirtualKeyCode::F15),
        "f16" => Ok(VirtualKeyCode::F16),
        "f17" => Ok(VirtualKeyCode::F17),
        "f18" => Ok(VirtualKeyCode::F18),
        "f19" => Ok(VirtualKeyCode::F19),
        "f20" => Ok(VirtualKeyCode::F20),
        "f21" => Ok(VirtualKeyCode::F21),
        "f22" => Ok(VirtualKeyCode::F22),
        "f23" => Ok(VirtualKeyCode::F23),
        "f24" => Ok(VirtualKeyCode::F24),
        "print" | "printscreen" => Ok(VirtualKeyCode::Snapshot),
        "scrolllock" | "scroll" => Ok(VirtualKeyCode::Scroll),
        "pause" => Ok(VirtualKeyCode::Pause),
        "insert" => Ok(VirtualKeyCode::Insert),
        "home" => Ok(VirtualKeyCode::Home),
        "delete" => Ok(VirtualKeyCode::Delete),
        "end" => Ok(VirtualKeyCode::End),
        "pagedown" | "pgdn" => Ok(VirtualKeyCode::PageDown),
        "pageup" | "pgup" => Ok(VirtualKeyCode::PageUp),
        "left" => Ok(VirtualKeyCode::Left),
        "up" => Ok(VirtualKeyCode::Up),
        "right" => Ok(VirtualKeyCode::Right),
        "down" => Ok(VirtualKeyCode::Down),
        "backspace" | "back" => Ok(VirtualKeyCode::Back),
        "return" | "enter" => Ok(VirtualKeyCode::Return),
        "space" | " " => Ok(VirtualKeyCode::Space),
        "compose" => Ok(VirtualKeyCode::Compose),
        "caret" => Ok(VirtualKeyCode::Caret),
        "numlock" => Ok(VirtualKeyCode::Numlock),
        "numpad0" | "num0" => Ok(VirtualKeyCode::Numpad0),
        "numpad1" | "num1" => Ok(VirtualKeyCode::Numpad1),
        "numpad2" | "num2" => Ok(VirtualKeyCode::Numpad2),
        "numpad3" | "num3" => Ok(VirtualKeyCode::Numpad3),
        "numpad4" | "num4" => Ok(VirtualKeyCode::Numpad4),
        "numpad5" | "num5" => Ok(VirtualKeyCode::Numpad5),
        "numpad6" | "num6" => Ok(VirtualKeyCode::Numpad6),
        "numpad7" | "num7" => Ok(VirtualKeyCode::Numpad7),
        "numpad8" | "num8" => Ok(VirtualKeyCode::Numpad8),
        "numpad9" | "num9" => Ok(VirtualKeyCode::Numpad9),
        "numpadadd" | "numadd" => Ok(VirtualKeyCode::NumpadAdd),
        "numpaddivide" | "numdivide" => Ok(VirtualKeyCode::NumpadDivide),
        "numpaddecimal" | "numdecimal" => Ok(VirtualKeyCode::NumpadDecimal),
        "numpadcomma" | "numcomma" => Ok(VirtualKeyCode::NumpadComma),
        "numpadenter" | "numenter" => Ok(VirtualKeyCode::NumpadEnter),
        "numpadequals" | "numequals" => Ok(VirtualKeyCode::NumpadEquals),
        "numpadmultiply" | "nummultiply" => Ok(VirtualKeyCode::NumpadMultiply),
        "numpadsubtract" | "numsubtract" => Ok(VirtualKeyCode::NumpadSubtract),
        "apostrophe" => Ok(VirtualKeyCode::Apostrophe),
        "asterisk" => Ok(VirtualKeyCode::Asterisk),
        "plus" => Ok(VirtualKeyCode::Plus),
        "comma" => Ok(VirtualKeyCode::Comma),
        "minus" => Ok(VirtualKeyCode::Minus),
        "period" => Ok(VirtualKeyCode::Period),
        "slash" => Ok(VirtualKeyCode::Slash),
        "colon" => Ok(VirtualKeyCode::Colon),
        "semicolon" => Ok(VirtualKeyCode::Semicolon),
        "at" => Ok(VirtualKeyCode::At),
        "backslash" => Ok(VirtualKeyCode::Backslash),
        "equals" => Ok(VirtualKeyCode::Equals),
        "grave" => Ok(VirtualKeyCode::Grave),
        "bracketleft" => Ok(VirtualKeyCode::LBracket),
        "bracketright" => Ok(VirtualKeyCode::RBracket),
        "tab" => Ok(VirtualKeyCode::Tab),
        _ => Err(()),
    }
}
