use bevy::{
    prelude::{info, App, Plugin, Res, ResMut, Resource, SystemSet},
    time::{FixedTimestep, Time},
};

pub(crate) struct ChronoPlugin;

impl Plugin for ChronoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeMultiplier(100))
            .insert_resource(Chrono::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(15.0))
                    .with_system(update_time),
            );
    }
}

#[derive(Resource)]
struct TimeMultiplier(u8);

#[derive(Resource, Default)]
pub(crate) struct Chrono {
    // ever increasing value
    tick: u32,
    // 0 -> 23, repeating
    pub(crate) hour: u32,
    // 0 -> 30, repeating
    pub(crate) day: u32,
    // 0 -> continous
    pub(crate) year: u32,
}

const TICKS_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;
const DAYS_PER_YEAR: u32 = 30;

fn update_time(time: Res<Time>, mut chrono: ResMut<Chrono>, multiplier: Res<TimeMultiplier>) {
    chrono.tick += multiplier.0 as u32;

    chrono.hour = hours_from_tick(chrono.tick);
    chrono.day = days_from_tick(chrono.tick);
    chrono.year = years_from_tick(chrono.tick);

    info!(
        "Tick: {:?} | Hour: {:?} | Day: {:?} | Year: {:?}",
        chrono.tick, chrono.hour, chrono.day, chrono.year
    );
}

fn hours_from_tick(tick: u32) -> u32 {
    (tick / TICKS_PER_HOUR).rem_euclid(HOURS_PER_DAY)
}

fn days_from_tick(tick: u32) -> u32 {
    (tick / TICKS_PER_HOUR / HOURS_PER_DAY).rem_euclid(DAYS_PER_YEAR)
}

fn years_from_tick(tick: u32) -> u32 {
    tick / TICKS_PER_HOUR / HOURS_PER_DAY / DAYS_PER_YEAR
}

// On a fixed update
// - Time progresses
// - Resources grow and update
// - Weather and rainfall
// - Pathfinding is updated

// As fast as possible
// - Interactions between agents and resources
// - animations
// - walking along pathfinding

#[cfg(test)]
mod tests {
    use super::hours_from_tick;
    use crate::chronos::TICKS_PER_HOUR;

    /// When we go over
    #[test]
    fn wrap_around() {
        assert_eq!(hours_from_tick(TICKS_PER_HOUR + 1), 1);
    }
}
