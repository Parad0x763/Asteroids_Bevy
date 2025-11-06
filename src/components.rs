use bevy::{prelude::{Color, Val, Resource, Deref, DerefMut, Component, Timer, Vec2, Vec3}, ecs::event::Event};

pub const SPAWN_PROTECTION_MS: u64 = 150;

// Scoreboard settings
pub const SCOREBOARD_FONT_SIZE: f32 = 28.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
pub const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
pub const SCORE_LABEL_TEXT: &str = "Score: ";
pub const DESTROY_ASTEROID_SCORE: u32 = 50;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreboardUi;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Asteroid;

#[derive(Event)]
pub struct PlayerDied;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct MovementPhysics {
    /// Linear speed in meters per second
    pub movement_speed: f32,
    /// Rotation speed in radians per second
    pub rotation_speed: f32,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct HealthComponent {
    pub max_health: f32,
    pub current_health: f32,
}

#[derive(Debug, Component, Clone, PartialEq, Default)]
pub struct Respawnable {
    pub number_of_lives: u32,
    pub spawn_protection: Timer,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Turret;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Collider;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, /*Deref, DerefMut*/)]
pub struct Velocity(pub Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

/// The value [`PhysicalTranslation`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput {
    pub movement: Vec2,
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct DidFixedTimestepRunThisFrame(pub bool);
