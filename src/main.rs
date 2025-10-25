use bevy::{color::palettes::tailwind, prelude::*};

use bevy_rand::prelude::{EntropyPlugin, WyRand};
use rand_core::RngCore;

mod components;
mod physics;
mod create_asteroids_plugin;
use std::time::Duration;

use crate::components::MovementPhysics;
use crate::components::PhysicalTranslation;
use crate::components::PreviousPhysicalTranslation;
use crate::components::Velocity;
use crate::components::AccumulatedInput;
use crate::components::Player;
use crate::components::DidFixedTimestepRunThisFrame;
use crate::physics::player_movement;
use crate::create_asteroids_plugin::AsteroidCreatePlugin;

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
                wait_duration: Duration::from_secs(1),
                message: "Created".to_string(),
            }
        ))
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, clear_fixed_timestep_flag)
        .add_systems(FixedPreUpdate, set_fixed_time_step_flag)
        .add_systems(FixedUpdate, player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    // Spawn the Player
    commands.spawn((
        Player,
        MovementPhysics {
            movement_speed: 500.0,                  // Meters per second
            rotation_speed: f32::to_radians(360.0), // Degress per second
        },
        Sprite::from_image(asset_server.load("ship/ship.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
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

