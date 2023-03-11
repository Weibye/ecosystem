// Display current game simulation speed
// Buttons to change game simulation speed

use bevy::{
    prelude::{
        default, App, AssetServer, BuildChildren, ButtonBundle, Changed, Color, Commands,
        Component, EventWriter, NodeBundle, Plugin, Query, Res, TextBundle, With,
    },
    text::{Text, TextStyle},
    time::Time,
    ui::{AlignItems, Interaction, JustifyContent, Size, Style, UiRect, Val},
};

use crate::chronos::{Chrono, SimulationSpeed, TimeMultiplierEvent};

pub(crate) struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_header)
            .add_system(set_simulation_speed)
            .add_system(update_hour_label)
            .add_system(update_day_label)
            .add_system(update_year_label)
            .add_system(update_speed_label);
    }
}

const FONT_SIZE: f32 = 16.0;

#[derive(Component)]
struct HourLabel;

#[derive(Component)]
struct DayLabel;

#[derive(Component)]
struct YearLabel;

#[derive(Component)]
struct SpeedLabel;

#[derive(Component)]
struct SimulationSpeedButton(SimulationSpeed);

fn set_simulation_speed(
    q: Query<(&Interaction, &SimulationSpeedButton), Changed<Interaction>>,
    mut writer: EventWriter<TimeMultiplierEvent>,
) {
    for (interaction, speed_setting) in &q {
        if matches!(*interaction, Interaction::Clicked) {
            writer.send(TimeMultiplierEvent(speed_setting.0));
        }
    }
}

fn build_header(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto/Roboto-Light.ttf");

    cmd.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // HEADER
        parent
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Px(30.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            })
            .with_children(|header| {
                // 3x buttons for simulation speed

                // Button group
                header
                    .spawn(NodeBundle { ..default() })
                    .with_children(|button_group| {
                        // Normal speed button
                        button_group
                            .spawn(ButtonBundle {
                                // button: bevy::prelude::Button,
                                style: Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // interaction: todo!(),
                                // focus_policy: todo!(),
                                background_color: Color::DARK_GRAY.into(),
                                // image: todo!(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text::from_section(
                                        "Normal",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: FONT_SIZE,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                            })
                            .insert(SimulationSpeedButton(SimulationSpeed::Normal));

                        // Medium speed button
                        button_group
                            .spawn(ButtonBundle {
                                // button: bevy::prelude::Button,
                                style: Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // interaction: todo!(),
                                // focus_policy: todo!(),
                                background_color: Color::DARK_GRAY.into(),
                                // image: todo!(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text::from_section(
                                        "Medium",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: FONT_SIZE,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                            })
                            .insert(SimulationSpeedButton(SimulationSpeed::Fast));

                        // Fast speed button
                        button_group
                            .spawn(ButtonBundle {
                                // button: bevy::prelude::Button,
                                style: Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    padding: UiRect::all(Val::Px(5.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                // interaction: todo!(),
                                // focus_policy: todo!(),
                                background_color: Color::DARK_GRAY.into(),
                                // image: todo!(),
                                ..default()
                            })
                            .with_children(|button| {
                                button.spawn(TextBundle {
                                    style: Style { ..default() },
                                    text: Text::from_section(
                                        "Fast",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: FONT_SIZE,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                });
                            })
                            .insert(SimulationSpeedButton(SimulationSpeed::SuperFast));
                    });

                // Labels for clock + day and year
                header.spawn(TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Speed:",
                        TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "1x",
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    },
                    SpeedLabel,
                ));

                header.spawn(TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Hour:",
                        TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "0",
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    },
                    HourLabel,
                ));

                header.spawn(TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Day:",
                        TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "0",
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    },
                    DayLabel,
                ));

                header.spawn(TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Year",
                        TextStyle {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });

                header.spawn((
                    TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "0",
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    },
                    YearLabel,
                ));
            });
    });
}

fn update_hour_label(mut q: Query<&mut Text, With<HourLabel>>, chrono: Res<Chrono>) {
    for mut text in &mut q {
        text.sections[0].value = format!("{:?}", chrono.hour);
    }
}

fn update_day_label(mut q: Query<&mut Text, With<DayLabel>>, chrono: Res<Chrono>) {
    for mut text in &mut q {
        text.sections[0].value = format!("{:?}", chrono.day_progression);
    }
}

fn update_year_label(mut q: Query<&mut Text, With<YearLabel>>, chrono: Res<Chrono>) {
    for mut text in &mut q {
        text.sections[0].value = format!("{:?}", chrono.year);
    }
}

fn update_speed_label(mut q: Query<&mut Text, With<SpeedLabel>>, time: Res<Time>) {
    for mut text in &mut q {
        text.sections[0].value = format!("{:?}x", time.relative_speed());
    }
}
