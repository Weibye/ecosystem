
use bevy::prelude::Plugin;
use rand::SeedableRng;
use rand::rngs::SmallRng;

pub(crate) struct Random(pub SmallRng);

/// A plugin providing randomness functionality.
pub(crate) struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Random(SmallRng::from_entropy()));
    }
}