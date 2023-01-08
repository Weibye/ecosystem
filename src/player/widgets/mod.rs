use bevy::{
    prelude::{App, Color, Plugin, Query},
    ui::{BackgroundColor, Interaction},
};

pub(crate) struct WidgetPlugin;

impl Plugin for WidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_interaction);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn update_interaction(mut q: Query<(&mut BackgroundColor, &Interaction)>) {
    for (mut background, interaction) in &mut q {
        background.0 = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON,
            Interaction::Hovered => HOVERED_BUTTON,
            Interaction::None => NORMAL_BUTTON,
        }
    }
}

// On the text label, there is a component that defines what data to bind to.
// Which component on the same entity.

// struct DataLabel {

// }

// fn update_labels {
// Query all data-labels
// magically fetch everything from world and inject the correct data into it.
// }
