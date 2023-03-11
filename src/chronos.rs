use bevy::{
    prelude::{App, EventReader, Plugin, Res, ResMut, Resource, SystemSet},
    time::{FixedTimestep, Time},
};
use leafwing_input_manager::Actionlike;

use crate::utils::{inverse_lerp, lerp};

pub(crate) struct ChronoPlugin;

impl Plugin for ChronoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimeMultiplierEvent>()
            // .insert_resource(TimeMultiplier(get_multiplier(SimulationSpeed::Normal)))
            .insert_resource(Chrono::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(15.0))
                    .with_system(update_time),
            )
            .add_system(update_relative_simulation_time);
    }
}

// #[derive(Resource)]
// pub(crate) struct TimeMultiplier(u8);

// impl TimeMultiplier {
//     pub(crate) fn value(&self) -> u8 {
//         self.0
//     }
// }

#[derive(Resource, Default)]
pub(crate) struct Chrono {
    /// How much time has progressed since the start, in seconds.
    elapsed: f64,
    /// Fraction of how much the current day has progressed.
    ///
    /// [0.0..1.0]
    pub(crate) day_progression: f64,
    /// Number of the current minute.
    ///
    /// [0..60]
    pub(crate) minute: u32,
    /// Number of the current hour.
    ///
    /// [0..24]
    pub(crate) hour: u32,
    /// Number of the current day.
    ///
    /// [0..30]
    pub(crate) day: u32,
    // 0 -> continous
    pub(crate) year: u32,
    pub(crate) period: f32,
}

#[derive(Actionlike, Debug, Clone, Copy)]
pub(crate) enum SimulationSpeed {
    Paused,
    Normal,
    Fast,
    SuperFast,
}

pub(crate) struct TimeMultiplierEvent(pub(crate) SimulationSpeed);

pub(crate) const fn get_multiplier(speed: SimulationSpeed) -> f32 {
    match speed {
        SimulationSpeed::Paused => 0.0,
        SimulationSpeed::Normal => 1.0,
        SimulationSpeed::Fast => 4.0,
        SimulationSpeed::SuperFast => 8.0,
    }
}

const REALTIME_SECONDS_IN_DAY: f64 = 60.0;
const HOURS_PER_DAY: u32 = 24;

const TICKS_PER_HOUR: u32 = 60;
const DAYS_PER_YEAR: u32 = 30;
const SECONDS_IN_DAY: f32 = 60.0;

fn update_relative_simulation_time(
    mut reader: EventReader<TimeMultiplierEvent>,
    mut time: ResMut<Time>,
) {
    // We only care about the newest event if there has been multiple this frame.
    if let Some(value) = reader.iter().last() {
        time.set_relative_speed(get_multiplier(value.0));
    }
}

fn update_time(mut chrono: ResMut<Chrono>, time: Res<Time>) {
    // How many ticks to advance per update rate of this system.
    chrono.elapsed = time.elapsed_seconds_f64();

    // 0 -> 1f how far into a day have we progressed?
    chrono.day_progression = inverse_lerp(
        (chrono.elapsed).rem_euclid(REALTIME_SECONDS_IN_DAY),
        0.0,
        REALTIME_SECONDS_IN_DAY,
    );

    chrono.hour = lerp(chrono.day_progression, 0.0, HOURS_PER_DAY as f64) as u32;

    // chrono.hour = hours_from_tick(chrono.tick);
    // chrono.day = days_from_tick(chrono.tick);
    // chrono.year = years_from_tick(chrono.tick);

    // chrono.period = time.delta_seconds();

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
