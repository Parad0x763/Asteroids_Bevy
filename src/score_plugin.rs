use bevy::prelude::*;

use crate::components::{SCORE_COLOR, SCORE_LABEL_TEXT, SCOREBOARD_FONT_SIZE, SCOREBOARD_TEXT_PADDING, Score, ScoreboardUi, TEXT_COLOR};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Score(0))
            .add_systems(Startup, init_scoreboard)
            .add_systems(Update, update_score);
    }
}

fn init_scoreboard(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new(SCORE_LABEL_TEXT),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(TEXT_COLOR),
        ScoreboardUi,
        Node {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        )],
    ));
}

fn update_score(
    score: Res<Score>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
}
