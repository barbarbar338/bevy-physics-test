use super::assets::*;
use bevy::{diagnostic::*, prelude::*, utils::*};
use std::fmt::Write;

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct FPSState {
    pub timer: Timer,
    pub update_now: bool,
}

impl Default for FPSState {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            update_now: true,
        }
    }
}

impl FPSState {
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    pub fn enabled(&self) -> bool {
        !self.timer.paused()
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct FPSText;

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FPSText>()
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .init_resource::<FPSState>()
            .add_startup_system(spawn_text)
            .add_system(update);
    }
}

fn update(
    mut text_query: Query<&mut Text, With<FPSText>>,
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    fps_state: Option<ResMut<FPSState>>,
) {
    if let Some(mut state) = fps_state {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            for mut text in text_query.iter_mut() {
                let value = &mut text.sections[0].value;
                value.clear();

                if !state.timer.paused() {
                    let fps_diags = diagnostics
                        .get(FrameTimeDiagnosticsPlugin::FPS)
                        .and_then(|fps| fps.average());

                    if let Some(fps) = fps_diags {
                        write!(value, "FPS: {:.0}\n", fps).unwrap();
                    } else {
                        write!(value, "FPS: N/A\n").unwrap();
                    }
                }
            }
        }
    }
}

fn spawn_text(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn(
            TextBundle::from_sections(vec![
                TextSection {
                    value: "FPS: N/A\n".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 16.0,
                        color: Color::GREEN,
                    },
                },
                TextSection {
                    value: "FPS Toggle: F\n".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 16.0,
                        color: Color::GREEN,
                    },
                },
                TextSection {
                    value: "Camera Toggle: T\n".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 16.0,
                        color: Color::GREEN,
                    },
                },
                TextSection {
                    value: "Movement: WASD\n".to_string(),
                    style: TextStyle {
                        font: assets.font.clone(),
                        font_size: 16.0,
                        color: Color::GREEN,
                    },
                },
            ])
            .with_text_alignment(TextAlignment::TOP_RIGHT)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(FPSText)
        .insert(Name::new("FPS Text"));
}
