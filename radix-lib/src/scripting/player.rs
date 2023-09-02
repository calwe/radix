use mlua::{IntoLua, UserData, Value};

use crate::player::Player;

impl UserData for Player {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get_speed", |_, this, _: ()| Ok(this.speed()));
        methods.add_method_mut("set_speed", |_, this, speed: f64| {
            this.set_speed(speed);
            Ok(())
        });
    }
}
