use std::{any::Any, cell::RefCell, rc::Rc};

use winit_input_helper::WinitInputHelper;

use crate::map::Map;

use paste::paste;

use super::{
    camera::Camera, player_controller::PlayerController, sprite::Sprite,
    sprite_transform::SpriteTransform, transform::Transform,
};

pub trait Component: Into<ComponentType> {
    fn update(&mut self, _input: &WinitInputHelper, _map: &Map) {}
    fn start(&mut self) {}
}

macro_rules! impl_component_types {
    ( $( ($x:ident) ),* ) => {
        pub enum ComponentType {
            $(
                $x(Rc<RefCell<$x>>),
            )*
        }

        paste! {
            impl Component for ComponentType {
                // macro all this?
                fn update(&mut self, input: &WinitInputHelper, map: &Map) {
                    match self {
                        $(
                            ComponentType::$x([<$x:lower>]) => [<$x:lower>].borrow_mut().update(input, map),
                        )*
                    }
                }

                fn start(&mut self) {
                    match self {
                        $(
                            ComponentType::$x([<$x:lower>]) => [<$x:lower>].borrow_mut().start(),
                        )*
                    }
                }
            }

            impl ComponentType {
                pub fn to_any(&self) -> &dyn Any {
                    match self {
                        $(
                            ComponentType::$x([<$x:lower>]) => [<$x:lower>],
                        )*
                    }
                }
            }
        }
    };
}

impl_component_types!(
    (Transform),
    (PlayerController),
    (Camera),
    (Sprite),
    (SpriteTransform)
);
