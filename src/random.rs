use bevy::prelude::{Plugin, Resource};
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(Resource)]
pub(crate) struct Random(pub SmallRng);

/// A plugin providing randomness functionality.
pub(crate) struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Random(SmallRng::from_entropy()));
    }
}
