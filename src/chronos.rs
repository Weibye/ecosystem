use bevy::prelude::{App, CoreSet, EventReader, IntoSystemConfig, Plugin, Res, ResMut, Resource};
use leafwing_input_manager::Actionlike;

pub(crate) struct ChronoPlugin;

// TODO: Calculate time on a fixed update

impl Plugin for ChronoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimeMultiplierEvent>()
            .insert_resource(TimeMultiplier(get_multiplier(SimulationSpeed::Normal)))
            .insert_resource(Chrono::default())
            // This should be updated in pre-update
            .add_system(update_time.in_base_set(CoreSet::First))
            .add_system(update_simulation_speed);
    }
}

#[derive(Resource)]
pub(crate) struct TimeMultiplier(u8);

impl TimeMultiplier {
    pub(crate) fn value(&self) -> u8 {
        self.0
    }
}

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

#[derive(Actionlike, Debug, Clone, Copy)]
pub(crate) enum SimulationSpeed {
    Paused,
    Normal,
    Fast,
    SuperFast,
}

pub(crate) struct TimeMultiplierEvent(pub(crate) SimulationSpeed);

pub(crate) const fn get_multiplier(speed: SimulationSpeed) -> u8 {
    match speed {
        SimulationSpeed::Paused => 0,
        SimulationSpeed::Normal => 1,
        SimulationSpeed::Fast => 4,
        SimulationSpeed::SuperFast => 8,
    }
}

const TICKS_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;
const DAYS_PER_YEAR: u32 = 30;

fn update_simulation_speed(
    mut reader: EventReader<TimeMultiplierEvent>,
    mut speed: ResMut<TimeMultiplier>,
) {
    // We only care about the newest event if there has been multiple this frame.
    if let Some(value) = reader.iter().last() {
        speed.0 = get_multiplier(value.0);
    }
}

fn update_time(mut chrono: ResMut<Chrono>, speed: Res<TimeMultiplier>) {
    // How many ticks to advance per update rate of this system.
    chrono.tick += speed.0 as u32;

    chrono.hour = hours_from_tick(chrono.tick);
    chrono.day = days_from_tick(chrono.tick);
    chrono.year = years_from_tick(chrono.tick);

    // info!(
    //     "Tick: {:?} | Hour: {:?} | Day: {:?} | Year: {:?}",
    //     chrono.tick, chrono.hour, chrono.day, chrono.year
    // );
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
