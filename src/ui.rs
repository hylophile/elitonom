use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{
    life::{
        noise::{AddNoiseEvent, RemoveNoiseEvent},
        step::StepLifeEvent,
        LifeConfig, StepTimer,
    },
    tree::{hat::HatMetaTileType, TreeConfig},
};

pub struct UIPlugin;

#[derive(Resource)]
struct UIState {
    birth: String,
    survival: String,
    update_interval: String,
    spectre: bool,
    meta_tile: HatMetaTileType,
    add_noise_percent: String,
    remove_noise_percent: String,
    stroke_width: String,
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState {
            birth: "3".to_string(),
            survival: "23".to_string(),
            update_interval: "0.01".to_string(),
            add_noise_percent: "10".to_string(),
            spectre: true,
            remove_noise_percent: "10".to_string(),
            stroke_width: "1".to_string(),
            meta_tile: HatMetaTileType::H,
        })
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_visuals_system)
        .add_system(ui_system);
    }
}
fn configure_visuals_system(mut egui_settings: ResMut<EguiSettings>) {
    egui_settings.scale_factor = 1.25;
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut life_config: ResMut<LifeConfig>,
    mut tree_config: ResMut<TreeConfig>,
    mut evt1: EventWriter<AddNoiseEvent>,
    mut evt2: EventWriter<RemoveNoiseEvent>,
    mut evt3: EventWriter<StepLifeEvent>,
    mut step_timer: ResMut<StepTimer>,
) {
    contexts.ctx_mut().set_visuals(egui::Visuals::light());

    egui::SidePanel::left("side_panel")
        .exact_width(150.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Settings");

            let running_label = if life_config.running { "Pause" } else { "Play" };
            if ui.button(running_label).clicked() {
                life_config.running = !life_config.running;
            }

            if ui.button("Step").clicked() {
                evt3.send(StepLifeEvent);
            }

            ui.horizontal(|ui| {
                ui.label("Update interval:");
                let response = ui.text_edit_singleline(&mut ui_state.update_interval);
                if response.changed() {
                    if let Ok(update_interval) = ui_state.update_interval.parse::<f32>() {
                        if update_interval > 0.0 {
                            *step_timer = StepTimer(Timer::from_seconds(
                                update_interval,
                                TimerMode::Repeating,
                            ));
                        }
                    }
                }
                ui.label("s");
            });

            ui.separator();

            ui.label("Shape:");

            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut ui_state.spectre, true, "Spectre")
                    .clicked()
                {
                    ui_state.spectre = true;
                    tree_config.spectre = true;
                };

                if ui
                    .radio_value(&mut ui_state.spectre, false, "Hat")
                    .clicked()
                {
                    ui_state.spectre = false;
                    tree_config.spectre = false;
                };
            });

            if !ui_state.spectre {
                ui.horizontal(|ui| {
                    if ui
                        .radio_value(&mut ui_state.meta_tile, HatMetaTileType::H, "H")
                        .clicked()
                    {
                        ui_state.meta_tile = HatMetaTileType::H;
                        tree_config.meta_tile = HatMetaTileType::H;
                    };
                    if ui
                        .radio_value(&mut ui_state.meta_tile, HatMetaTileType::T, "T")
                        .clicked()
                    {
                        ui_state.meta_tile = HatMetaTileType::T;
                        tree_config.meta_tile = HatMetaTileType::T;
                    };

                    if ui
                        .radio_value(&mut ui_state.meta_tile, HatMetaTileType::P, "P")
                        .clicked()
                    {
                        ui_state.meta_tile = HatMetaTileType::P;
                        tree_config.meta_tile = HatMetaTileType::P;
                    };
                    if ui
                        .radio_value(&mut ui_state.meta_tile, HatMetaTileType::F, "F")
                        .clicked()
                    {
                        ui_state.meta_tile = HatMetaTileType::F;
                        tree_config.meta_tile = HatMetaTileType::F;
                    };
                });
            };

            ui.separator();

            ui.label("Rules:");

            ui.horizontal(|ui| {
                ui.label("Birth:");
                let response = ui.text_edit_singleline(&mut ui_state.birth);
                if response.changed() {
                    let mut n = [false, false, false, false, false, false, false, false];
                    ui_state
                        .birth
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .filter(|x| x < &8)
                        .for_each(|d| n[d as usize] = true);
                    life_config.birth = n;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Survival:");
                let response = ui.text_edit_singleline(&mut ui_state.survival);
                if response.changed() {
                    let mut n = [false, false, false, false, false, false, false, false];
                    ui_state
                        .survival
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .filter(|x| x < &8)
                        .for_each(|d| n[d as usize] = true);
                    life_config.survival = n;
                }
            });

            ui.separator();

            ui.label("Cell manipulation:");

            ui.horizontal(|ui| {
                if ui.button("Fill all").clicked() {
                    evt1.send(AddNoiseEvent { fraction: 1.01 });
                };
                if ui.button("Clear all").clicked() {
                    evt2.send(RemoveNoiseEvent { fraction: 1.01 });
                };
            });

            ui.horizontal(|ui| {
                if ui.button("Add Noise").clicked() {
                    evt1.send(AddNoiseEvent {
                        fraction: life_config.add_noise_percent,
                    });
                }
                let response = ui.text_edit_singleline(&mut ui_state.add_noise_percent);
                if response.changed() {
                    life_config.add_noise_percent = ui_state
                        .add_noise_percent
                        .parse::<f32>()
                        .map(|f| f / 100.0)
                        .unwrap_or(0.1);
                }
                ui.label("%");
            });

            ui.horizontal(|ui| {
                if ui.button("Remove Noise").clicked() {
                    evt2.send(RemoveNoiseEvent {
                        fraction: life_config.remove_noise_percent,
                    });
                }
                let response = ui.text_edit_singleline(&mut ui_state.remove_noise_percent);
                if response.changed() {
                    life_config.remove_noise_percent = ui_state
                        .remove_noise_percent
                        .parse::<f32>()
                        .map(|f| f / 100.0)
                        .unwrap_or(0.1);
                }
                ui.label("%");
            });
            ui.horizontal(|ui| {
                ui.label("Drawing stroke width:");
                if ui
                    .text_edit_singleline(&mut ui_state.stroke_width)
                    .changed()
                {
                    life_config.stroke_width = ui_state.stroke_width.parse::<usize>().unwrap_or(1)
                }
            });
        });
}
