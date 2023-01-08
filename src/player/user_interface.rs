// Display current game simulation speed
// Buttons to change game simulation speed

use bevy::{
    prelude::{
        default, App, AssetServer, BuildChildren, Color, Commands, Component, NodeBundle, Plugin,
        Query, Res, TextBundle, With, ButtonBundle,
    },
    text::{Text, TextStyle},
    ui::{Size, Style, UiRect, Val},
};

use crate::chronos::Chrono;

pub(crate) struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_header)
            .add_system(update_hour_label)
            .add_system(update_day_label)
            .add_system(update_year_label);
    }
}

const FONT_SIZE: f32 = 16.0;

#[derive(Component)]
struct HourLabel;

#[derive(Component)]
struct DayLabel;

#[derive(Component)]
struct YearLabel;

fn build_header(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Roboto/Roboto-Light.ttf");

    cmd.spawn(NodeBundle {
        style: Style {
            // padding: UiRect::all(Val::Px(10.0)),
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
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            })
            .with_children(|header| {
                // 3x buttons for simulation speed
                header.spawn(ButtonBundle {
                    // button: bevy::prelude::Button,
                    // style: todo!(),
                    // interaction: todo!(),
                    // focus_policy: todo!(),
                    // background_color: todo!(),
                    // image: todo!(),
                    ..default()
                }).with_children(| button | {
                    button.spawn(TextBundle {
                        style: Style { ..default() },
                        text: Text::from_section(
                            "Time:",
                            TextStyle {
                                font: font.clone(),
                                font_size: FONT_SIZE,
                                color: Color::WHITE,
                            },
                        ),
                        ..default()
                    });
                });



                // Labels for clock + day and year
                header.spawn(TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Time:",
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
        text.sections[0].value = format!("{:?}", chrono.day);
    }
}

fn update_year_label(mut q: Query<&mut Text, With<YearLabel>>, chrono: Res<Chrono>) {
    for mut text in &mut q {
        text.sections[0].value = format!("{:?}", chrono.year);
    }
}
