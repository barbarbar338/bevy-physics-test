mod assets;
mod controls;
mod duck;
mod fps;
mod utils;

use assets::*;
use bevy::{prelude::*, window::*};
use bevy_inspector_egui::*;
use bevy_rapier3d::prelude::*;
use controls::*;
use duck::*;
use fps::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy 3D Physics".to_string(),
                resizable: false,
                cursor_visible: false,
                cursor_grab_mode: CursorGrabMode::Confined,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(AssetsPlugin)
        .add_plugin(ControlsPlugin)
        .add_plugin(FPSPlugin)
        .add_plugin(DuckPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: 20.0,
                max_y: 1.0,
                max_z: 20.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .with_children(|commands| {
            commands.spawn(Collider::cuboid(10.0, 1.0, 10.0));
        })
        .insert(Transform::from_xyz(-10.0, 0.0, -10.0))
        .insert(Name::new("Ground"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
