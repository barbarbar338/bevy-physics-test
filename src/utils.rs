use bevy::prelude::*;

pub fn movement_axis(input: &Res<Input<KeyCode>>, plus: KeyCode, minus: KeyCode) -> f32 {
    let mut axis = 0.0;
    if input.pressed(plus) {
        axis += 1.0;
    }
    if input.pressed(minus) {
        axis -= 1.0;
    }
    axis
}

pub fn strafe_vector(rotation: &Quat) -> Vec3 {
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

pub fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = rotation.mul_vec3(Vec3::Z).normalize();
    let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
    f_flattened
}
