use bevy::ecs::query;
use bevy::ecs::system::IntoResult;
use bevy::{math::ops, prelude::*};

use crate::components::MovementPhysics;
use crate::Velocity;
use crate::AccumulatedInput;
use crate::Player;

const BOUNDS: Vec2 = Vec2::new(1280.0, 720.0);
const ROTATION_FACTOR: f32 = 0.5;
const MOVEMENT_FACTOR: f32 = 1.0;

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Transform, &mut MovementPhysics), With<Player>>,
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
}

pub fn adjust_entity_to_bounds(
     mut query: Query<(&mut Transform, &Velocity)>
) {
     for (mut transform, _) in &mut query {
          if transform.translation.x > BOUNDS.x / 2.0 {
               transform.translation.x -= BOUNDS.x;
          }
          if transform.translation.x < -BOUNDS.x / 2.0 {
               transform.translation.x += BOUNDS.x;
          }
          if transform.translation.y > BOUNDS.y / 2.0 {
               transform.translation.y -= BOUNDS.y;
          }
          if transform.translation.y < -BOUNDS.y / 2.0 {
               transform.translation.y += BOUNDS.y;
          }
     }
}
