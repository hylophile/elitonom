pub mod init;
pub mod noise;
pub mod step;

use self::{
    init::{gen_neighbors, init_life},
    noise::{add_noise, AddNoiseEvent},
    step::step_life,
};
use crate::constants::FIXED_TIMESTEP;
use bevy::app::StartupSet::PostStartup;
use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Debug)]
pub struct LifeConfig {
    pub running: bool,
    pub birth: HashSet<u32>,
    pub survival: HashSet<u32>,
    pub update_interval: f32,
}

#[derive(Resource, Debug)]
pub struct StepTimer(pub Timer);

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        let update_interval = 0.25;
        app.add_system(gen_neighbors)
            // .insert_resource(life_state)
            .insert_resource(LifeConfig {
                running: false,
                birth: HashSet::from([3]),
                survival: HashSet::from([2, 3]),
                update_interval,
            })
            .insert_resource(StepTimer(Timer::from_seconds(
                update_interval,
                TimerMode::Repeating,
            )))
            .add_event::<AddNoiseEvent>()
            .add_startup_system(init_life.in_base_set(PostStartup))
            .add_system(add_noise)
            .add_system(step_life.run_if(life_running));
        // .add_system(
        //     step_life
        //         .run_if(life_running)
        //         .in_schedule(CoreSchedule::FixedUpdate),
        // )
        // .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP));
    }
}

fn life_running(config: Res<LifeConfig>) -> bool {
    config.running
}
