use bevy::{color::palettes::tailwind, prelude::*};
use bevy_rand::prelude::{EntropyPlugin, WyRand};
use std::time::Duration;

mod components;
mod physics;
mod create_asteroids_plugin;
mod handle_turret_plugin;
mod collision;

use crate::components::{SPAWN_PROTECTION_MS, AccumulatedInput, DidFixedTimestepRunThisFrame, Respawnable, MovementPhysics, PhysicalTranslation, Player, PreviousPhysicalTranslation, Velocity};
use crate::{physics::{adjust_entity_to_bounds, player_movement}, create_asteroids_plugin::AsteroidCreatePlugin, handle_turret_plugin::{TurretShotPlugin}, collision::CollisionPlugin};

const PLAYER_SPEED: f32 = 350.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .init_resource::<DidFixedTimestepRunThisFrame>()
        .add_plugins(EntropyPlugin::<WyRand>::default()) // Enables PRNG algorithm
        .add_plugins((
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Asteroids".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280, 720).into(), // older versions of Bevy used f32, now u32 is used
                    ..default()
                }),
                ..default()}),
            AsteroidCreatePlugin {
                wait_duration: Duration::from_secs(1), //TODO: change this to where the plugin manages the time so that it can change
            },
            TurretShotPlugin,
            CollisionPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, clear_fixed_timestep_flag)
        .add_systems(FixedPreUpdate, set_fixed_time_step_flag)
        .add_systems(FixedUpdate, (player_movement, adjust_entity_to_bounds, tick_respawnable))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Player,
        MovementPhysics {
            movement_speed: PLAYER_SPEED,               // Meters per second
            rotation_speed: f32::to_radians(360.0),     // Degrees per second
        },
        Respawnable {
            number_of_lives: 3,
            spawn_protection: Timer::new(Duration::from_millis(SPAWN_PROTECTION_MS), TimerMode::Once),
        },
        Sprite::from_image(asset_server.load("ship/ship.png")),
        Transform::default(),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
}

fn tick_respawnable(time: Res<Time>, mut query: Query<&mut Respawnable>) {
    for mut respawnable in query.iter_mut() {
        respawnable.spawn_protection.tick(time.delta());
    }
}

// GAME TIME FUNCTIONS

/// reset flag at the start of every frame
fn clear_fixed_timestep_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = false;
}

/// Set the flag during each fixed timestep.
fn set_fixed_time_step_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = true;
}

fn did_fixed_timestep_run_this_frame(
    did_fixed_timestep_run_this_frame: Res<DidFixedTimestepRunThisFrame>,
) -> bool {
    did_fixed_timestep_run_this_frame.0
}

// Clear the input after it was processed in the fixed timestep.
fn clear_input(mut input: Single<&mut AccumulatedInput>) {
    **input = AccumulatedInput::default();
}

/// Advance the physics simulation by one fixed timestep. This may run zero or multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be `Res<Time<Fixed>>` automatically.
/// We are being explicit here for clarity.
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &Velocity,
    )>,
) {
    for (mut current_physical_translation, mut previous_physical_translation, velocity) in
        query.iter_mut()
    {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();
    }
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}

// Sync the camera's position with the player's interpolated position
fn translate_camera(
    mut camera: Single<&mut Transform, With<Camera>>,
    player: Single<&Transform, (With<AccumulatedInput>, Without<Camera>)>,
) {
    camera.translation = player.translation;
}

