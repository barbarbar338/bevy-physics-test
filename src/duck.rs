use super::assets::*;
use super::controls::*;
use super::utils::*;
use bevy::{input::mouse::*, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Reflect, Component)]
#[reflect(Component)]
pub struct Duck {
    pub pitch: f32,
    pub yaw: f32,
    pub speed: f32,
    pub gravity: f32,
    pub sensitivity: f32,
    pub enabled: bool,
}

impl Default for Duck {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            speed: 5.0,
            gravity: -9.81,
            sensitivity: 3.0,
            enabled: true,
        }
    }
}

pub struct DuckPlugin;

impl Plugin for DuckPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Duck>()
            .add_startup_system(spawn_duck)
            .add_system(duck_physics)
            .add_system(mouse_motion_system);
    }
}

fn spawn_duck(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(SceneBundle {
            scene: assets.duck.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule(
            Vec3::new(0.0, 0.4, 0.0),
            Vec3::new(0.0, 1.2, 0.0),
            0.5,
        ))
        .insert(KinematicCharacterController::default())
        .insert(Transform::from_xyz(0.0, 5.0, 0.0))
        .insert(Duck::default())
        .with_children(|commands| {
            commands
                .spawn(Camera3dBundle::default())
                .insert(Transform::from_xyz(2.0, 2.0, 10.0))
                .insert(Name::new("Camera"));
        })
        .insert(Name::new("Duck"));
}

fn duck_physics(
    mut ducks: Query<(&mut KinematicCharacterController, &Duck, &Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    controls: Res<Controls>,
) {
    let (axis_h, axis_v) = (
        movement_axis(&keyboard_input, controls.key_right, controls.key_left)
            * time.delta_seconds(),
        movement_axis(&keyboard_input, controls.key_backward, controls.key_forward)
            * time.delta_seconds(),
    );

    for (mut controller, duck, transform) in &mut ducks {
        if !duck.enabled {
            continue;
        }

        let rotation = transform.rotation;
        let accel: Vec3 = (strafe_vector(&rotation) * axis_h * duck.speed)
            + (forward_walk_vector(&rotation) * axis_v * duck.speed)
            + (Vec3::Y * time.delta_seconds() * duck.gravity);

        controller.translation = Some(accel);
    }
}

fn mouse_motion_system(
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut Duck, &mut Transform)>,
    time: Res<Time>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }

        options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
        options.pitch += delta.y * options.sensitivity * time.delta_seconds();

        options.pitch = options.pitch.clamp(-89.0, 89.9);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw_radians)
            * Quat::from_axis_angle(-Vec3::X, pitch_radians);
    }
}
