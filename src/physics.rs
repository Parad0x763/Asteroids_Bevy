use bevy::{math::ops, prelude::*};

use crate::Velocity;
use crate::AccumulatedInput;
use crate::Player;

const BOUNDS: Vec2 = Vec2::new(1280.0, 720.0);
const ROTATION_FACTOR: f32 = 0.5;
const MOVEMENT_FACTOR: f32 = 1.0;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut Player), With<Player>>,
) {
   let (mut transform, ship) = player.into_inner();

   let mut rotation_factor: f32 = 0.0;
   let mut movement_factor: f32 = 0.0;

   if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += ROTATION_FACTOR;
   }

   if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= ROTATION_FACTOR;
   }

   if keyboard_input.pressed(KeyCode::KeyW) {
        movement_factor += MOVEMENT_FACTOR;
   }

   // Update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
   transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_secs());

   // Get the ship's forward vector by applying the current rotation to the ships initial facing vector
   let movement_direction = transform.rotation * Vec3::Y;
   // Get the distance the ship will move based on the direction, the ship's movement speed and delta time
   let movement_distance: f32 = movement_factor * ship.movement_speed * time.delta_secs();
   // Create the change in translation using the new movement direction and distance
   let translation_delta = movement_direction * movement_distance;
   // Update the ship translation with our new translation delta
   transform.translation += translation_delta;

   // Bound the ship within the invisible level bounds
   let extents = Vec3::from((BOUNDS / 2.0, 0.0));
   transform.translation = transform.translation.min(extents).max(-extents);
}
