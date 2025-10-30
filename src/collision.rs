use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{Player, components::{Asteroid, Turret}};

pub struct CollisionPlugin;

pub const SPRITE_DIAMETER: f32 = 45.254833959; // the ship is a 32x32 sprite -> sqrt(32^2 + 32^2)
const SPRITE_CENTER_FACTOR: f32 = 2.0;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, check_for_collisions);
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    player: Single<(Entity, &Transform), (With<Player>, Without<Asteroid>, Without<Turret>)>,
    shots: Query<(Entity, &Transform), (With<Turret>, Without<Asteroid>, Without<Player>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Turret>, Without<Player>)>,
) {
    let (p_entity, p_transform) = player.into_inner();
    
    for (a_entity, a_transform) in asteroids.iter() {
        let b_player_collision: bool = has_collided(
            BoundingCircle::new(p_transform.translation.truncate(), SPRITE_DIAMETER / SPRITE_CENTER_FACTOR),
            Aabb2d::new(
                a_transform.translation.truncate(),
                a_transform.scale.truncate() / SPRITE_CENTER_FACTOR,
            )
        );
        
        if b_player_collision {            
            commands.entity(p_entity).despawn();
            commands.entity(a_entity).despawn();
        }
        
        for (s_entity, s_transform) in shots.iter() {
            let b_shot_collision: bool = has_collided(
                BoundingCircle::new(s_transform.translation.truncate(), SPRITE_DIAMETER / SPRITE_CENTER_FACTOR),
                Aabb2d::new(
                    a_transform.translation.truncate(),
                    a_transform.scale.truncate() / SPRITE_CENTER_FACTOR
                )
            );
            
            if b_shot_collision {
                commands.entity(s_entity).despawn();
                commands.entity(a_entity).despawn();
            }
        }
    }
}

fn has_collided(
    bounding_circle: BoundingCircle,
    bounding_box: Aabb2d
) -> bool {
    if !bounding_circle.intersects(&bounding_box) {
        return false;
    }
    
    return true;
}
