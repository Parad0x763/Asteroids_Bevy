use rand::Rng;

use bevy::prelude::*;
use bevy_rand::prelude::{WyRand, GlobalRng};
use core::time::Duration;

use crate::components::{Asteroid, Velocity, PlayerDied};
use crate::{PreviousPhysicalTranslation, PhysicalTranslation, MovementPhysics, sprite_paths::ASTEROID_SPRITE_PATH};

/// Asteroid Creation Plugin
pub struct AsteroidCreatePlugin {
    pub wait_duration: Duration,
    //TODO: Add properties to scale the asteroids so that 2x asteroids can be created
    // pub is_large_asteroid: bool,
}

impl Plugin for AsteroidCreatePlugin {
    fn build(&self, app: &mut App) {
        let state: AsteroidCreateState = AsteroidCreateState {
            timer: Timer::new(self.wait_duration, TimerMode::Repeating),
            can_spawn_asteroids: true,
        };
        app
            .insert_resource(state)
            .add_systems(Update, spawn_more_asteroids.run_if(can_spawn_asteroids))
            .add_systems(FixedUpdate, update_movement)
            .add_observer(remove_asteroids_on_player_death);
    }
}

#[derive(Resource)]
pub struct AsteroidCreateState {
    pub timer: Timer,
    pub can_spawn_asteroids: bool,
}

fn can_spawn_asteroids(state: Res<AsteroidCreateState>) -> bool {
    return state.can_spawn_asteroids;
}

pub fn spawn_more_asteroids(
    mut commands: Commands,
    mut state: ResMut<AsteroidCreateState>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>
) {
    let movement_speed_rng = rng.random_range(10.0..1500.0);
    let rotation_speed_rng = rng.random_range(0.0..359.0);

    let transform_x_rng = rng.random_range(-640.0..640.0);
    let transform_y_rng = rng.random_range(-360.0..360.0);

    let velocity_x_rng = rng.random_range(-1000.0..1000.0);
    let velocity_y_rng = rng.random_range(-1000.0..1000.0);

    if state.timer.tick(time.delta()).is_finished() {
        // TODO: Make adjustments to the ranges to make the asteroids more fun to interacte with, they are really fast right now
        commands.spawn((
            Asteroid,
            MovementPhysics {
                movement_speed: movement_speed_rng,                  // Meters per second
                rotation_speed: f32::to_radians(rotation_speed_rng), // Degress per second
            },
            Sprite::from_image(asset_server.load(ASTEROID_SPRITE_PATH)),
            Transform::from_xyz(transform_x_rng, transform_y_rng, 0.0),
            Velocity(Vec3 { x: velocity_x_rng, y: velocity_y_rng, z: 0.0 }),
            PhysicalTranslation::default(),
            PreviousPhysicalTranslation::default(),
        ));
    }
}

fn update_movement(mut query: Query<(&mut Transform, &Velocity), With<Asteroid>>, time: Res<Time>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_secs();
    }
}

fn remove_asteroids_on_player_death(
    _player_died: On<PlayerDied>,
    mut commands: Commands,
    mut query: Query<Entity, With<Asteroid>>,
    mut state: ResMut<AsteroidCreateState>,
) {
    state.can_spawn_asteroids = false;
    for a_entity in query.iter_mut() {
        commands.entity(a_entity).despawn();
    }
}
