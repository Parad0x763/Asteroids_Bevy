use bevy::{
    math::{bounding::{Aabb2d, BoundingCircle, IntersectsVolume}},
    prelude::*,
};

use crate::{Duration, Player, components::{PlayerDied, Asteroid, DESTROY_ASTEROID_SCORE, Respawnable, SPAWN_PROTECTION_MS, Score, Turret}};

pub struct CollisionPlugin;

pub const SPRITE_DIAMETER: f32 = 45.254833959; // the ship is a 32x32 sprite -> sqrt(32^2 + 32^2)
const SPRITE_CENTER_FACTOR: f32 = 2.0;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, check_for_collisions);
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut score: ResMut<Score>,
    player: Single<(Entity, &mut Transform, &mut Respawnable), (With<Player>, Without<Asteroid>, Without<Turret>)>,
    shots: Query<(Entity, &Transform), (With<Turret>, Without<Asteroid>, Without<Player>)>,
    asteroids: Query<(Entity, &Transform), (With<Asteroid>, Without<Turret>, Without<Player>)>,
) {
    let (p_entity, mut p_transform, mut p_respawn) = player.into_inner();
    
    for (a_entity, a_transform) in asteroids.iter() {
        let b_player_collision: bool = has_collided(
            BoundingCircle::new(p_transform.translation.truncate(), SPRITE_DIAMETER / SPRITE_CENTER_FACTOR),
            Aabb2d::new(
                a_transform.translation.truncate(),
                a_transform.scale.truncate() / SPRITE_CENTER_FACTOR,
            )
        );
        
        if b_player_collision {
            if p_respawn.spawn_protection.is_finished() {
                p_respawn.number_of_lives -= 1;
                
                if p_respawn.number_of_lives < 1 {
                    commands.entity(p_entity).despawn();
                    commands.trigger(PlayerDied);
                } else {
                    p_transform.translation = Vec3::ZERO;
                    p_transform.rotation = Quat::IDENTITY;
                    p_respawn.spawn_protection = Timer::new(Duration::from_millis(SPAWN_PROTECTION_MS), TimerMode::Once);
                }
            }
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
                **score += DESTROY_ASTEROID_SCORE;
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
