use bevy::{app::{App, Plugin}, ecs::{system::Commands}, prelude::*};

use crate::components::{HealthComponent, MovementPhysics, Player, Turret, Velocity};
use crate::{PhysicalTranslation, PreviousPhysicalTranslation, sprite_paths::SHOT_SPRITE_PATH};

const SHOT_HEALTH: f32 = 1.0;
const SHOT_SPEED: f32 = 1000.0;

/// Create Turret Shot Plugin
pub struct TurretShotPlugin;

impl Plugin for TurretShotPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_turret_shot)
            .add_systems(FixedUpdate, update_shot_movement);
    }
}

pub fn spawn_turret_shot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Single<(&Transform, &mut MovementPhysics), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (transform, movement_physics) = player.into_inner();
        commands.spawn((
            Turret,
            MovementPhysics {
                movement_speed: SHOT_SPEED,
                rotation_speed: movement_physics.rotation_speed,
            },
            HealthComponent {
                max_health: SHOT_HEALTH,
                current_health: SHOT_HEALTH,
            },
            Velocity(SHOT_SPEED * (transform.rotation * Vec3::X)),
            Transform {
                translation: transform.translation,
                rotation: transform.rotation.normalize(),
                scale: transform.scale,
            },
            PhysicalTranslation(transform.translation.into()),
            PreviousPhysicalTranslation(transform.translation.into()),
            Sprite::from_image(asset_server.load(SHOT_SPRITE_PATH))
        ));
    }
}

fn update_shot_movement(
    mut query: Query<(&mut Transform, &MovementPhysics), With<Turret>>, time: Res<Time>
) {
    for (mut transform, movement_physics) in query.iter_mut() {
        let forward_vector = transform.rotation * Vec3::Y;
        transform.translation += movement_physics.movement_speed * forward_vector * time.delta_secs();
    }
}
