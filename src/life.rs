pub mod draw;
pub mod init;
pub mod noise;
pub mod step;

use self::{
    draw::draw,
    init::gen_neighbors,
    noise::{add_noise, remove_noise, AddNoiseEvent, RemoveNoiseEvent},
    step::{step_life, tick_life, StepLifeEvent},
};

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct LifeConfig {
    pub running: bool,
    pub birth: [bool; 8],
    pub survival: [bool; 8],
    pub update_interval: f32,
    pub add_noise_percent: f32,
    pub remove_noise_percent: f32,
    pub stroke_width: usize,
}

#[derive(Resource, Debug)]
pub struct StepTimer(pub Timer);

pub struct LifePlugin;

impl Plugin for LifePlugin {
    fn build(&self, app: &mut App) {
        let update_interval = 0.01;
        app.add_system(gen_neighbors)
            .insert_resource(LifeConfig {
                running: false,
                birth: [false, false, false, true, false, false, false, false],
                survival: [false, false, true, true, false, false, false, false],
                update_interval,
                add_noise_percent: 0.1,
                remove_noise_percent: 0.1,
                stroke_width: 1,
            })
            .insert_resource(StepTimer(Timer::from_seconds(
                update_interval,
                TimerMode::Repeating,
            )))
            .add_event::<AddNoiseEvent>()
            .add_event::<RemoveNoiseEvent>()
            .add_event::<StepLifeEvent>()
            .add_system(add_noise)
            .add_system(remove_noise)
            .add_system(step_life)
            .add_system(tick_life.run_if(life_running))
            .add_system(draw);
    }
}

fn life_running(config: Res<LifeConfig>) -> bool {
    config.running
}
