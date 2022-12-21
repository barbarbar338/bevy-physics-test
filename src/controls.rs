use super::duck::*;
use super::fps::*;
use bevy::{prelude::*, window::*};
use std::process::*;

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct Controls {
    pub camera_toggler: KeyCode,
    pub terminate: KeyCode,
    pub fps_toggler: KeyCode,
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            camera_toggler: KeyCode::T,
            terminate: KeyCode::Escape,
            fps_toggler: KeyCode::F,
            key_forward: KeyCode::W,
            key_backward: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
        }
    }
}

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>().add_system(controls_system);
    }
}

fn controls_system(
    mut windows: ResMut<Windows>,
    mut duck_query: Query<&mut Duck>,
    mut fps: ResMut<FPSState>,
    controls: Res<Controls>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();
    let mut duck = duck_query.single_mut();

    if keyboard_input.just_pressed(controls.camera_toggler) {
        duck.enabled = !duck.enabled;

        window.set_cursor_grab_mode(if duck.enabled {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        });
        window.set_cursor_visibility(!duck.enabled);
    }

    if keyboard_input.just_pressed(controls.terminate) {
        exit(0);
    }

    if keyboard_input.just_pressed(controls.fps_toggler) {
        if fps.enabled() {
            fps.disable();
        } else {
            fps.enable();
        }
    }
}
