use bevy::{
    app::Update, camera::{Camera2d, RenderTarget, visibility::RenderLayers}, input::keyboard::{Key, KeyboardInput}, prelude::{App, Camera, Commands, Entity, IntoScheduleConfigs, MessageReader, Node, On, Plugin, PositionType, Res, Single, Text, Text2d, TextFont, Window, Without, default, px}, sprite::Text2dShadow, ui::{UiTargetCamera, widget::TextShadow}, window::{PrimaryWindow, WindowRef}
};

use crate::{rusqlite_database::insert_score, components::{PlayerDied, Score}};

const FONT_SIZE: f32 = 65.0;
const TOP_LEFT_PX: i32 = 12;
const INPUT_BOX_TITLE: &str = "Input Box";
const INPUT_NAME_LABEL: &str = "Enter Name:";

pub struct InputBoxPlugin;

impl Plugin for InputBoxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_observer(spawn_input_box)
            .add_systems(
                Update, (
                    listen_for_keyboard_input_events,
                ).after(spawn_input_box));
    }
}

/// Spawns a new window that takes user input
fn spawn_input_box(
    _player_died: On<PlayerDied>,
    mut commands: Commands,
) {

    let input_box_window = commands.spawn(Window {
        title: INPUT_BOX_TITLE.to_owned(),
        focused: true,
        ..default()
    })
    .id();
    
    let input_box_camera = commands.spawn((
        Camera2d,
        RenderLayers::layer(1),
        Camera {
            target: RenderTarget::Window(WindowRef::Entity(input_box_window)),
            ..default()
        },
    ))
    .id();

    let text_font = TextFont::from_font_size(FONT_SIZE);

    let node = Node {
        position_type: PositionType::Absolute,
        top: px(TOP_LEFT_PX),
        left: px(TOP_LEFT_PX),
        ..default()
    };
    
    commands
        .spawn((node, UiTargetCamera(input_box_camera)))
        .with_child((
            Text::new(INPUT_NAME_LABEL),
            text_font.clone(),
            TextShadow::default(),
        ));
    
    commands.spawn((
        Text2d::new(""),
        text_font.clone(),
        Text2dShadow::default(),
        RenderLayers::layer(1),
    ));
}

/// Reads keyboard input and uses that input for saving the player's score
/// Calls `rusqlite_database.rs` function `inser_score` to save the score
fn listen_for_keyboard_input_events(
    mut commands: Commands,
    mut keyboard_input_reader: MessageReader<KeyboardInput>,
    edit_text: Single<(&mut Text2d, &TextFont), Without<Node>>,
    input_box: Single<(Entity, &Window), Without<PrimaryWindow>>,
    score: Res<Score>,
) {
    let (mut text, _style) = edit_text.into_inner();
    let (input_window_entity, _) = input_box.into_inner();
    for keyboard_input in keyboard_input_reader.read() {
        // Only tigger changes when the key is first pressed
        if !keyboard_input.state.is_pressed() {
            continue;
        }

        match (&keyboard_input.logical_key, &keyboard_input.text) {
            (Key::Enter, _) => {
                if text.is_empty() {
                    continue;
                }
                
                let _ = insert_score(&text.0, score.0);
                commands.entity(input_window_entity).despawn();
            }
            (Key::Backspace, _) => {
                text.pop();
            }
            (_, Some(inserted_text)) => {
                // Make sure the text doesn't have any control characters,
                // Which can happpen when keys like Escape are pressed
                if inserted_text.chars().all(is_printable_char) {
                    text.push_str(inserted_text);
                }
            }
            _ => continue,
        }
    }
}

// this logic is taken from egui-winit:
// https://github.com/emilk/egui/blob/adfc0bebfc6be14cee2068dee758412a5e0648dc/crates/egui-winit/src/lib.rs#L1014-L1024
fn is_printable_char(chr: char) -> bool {
    let is_in_private_use_area = ('\u{e000}'..='\u{f8ff}').contains(&chr)
        || ('\u{f0000}'..='\u{ffffd}').contains(&chr)
        || ('\u{100000}'..='\u{10fffd}').contains(&chr);

    !is_in_private_use_area && !chr.is_ascii_control()
}
