use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{
    life::{
        noise::{AddNoiseEvent, RemoveNoiseEvent},
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
    // levels: String,
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
            // levels: "5".to_string(),
            add_noise_percent: "10".to_string(),
            remove_noise_percent: "10".to_string(),
            stroke_width: "1".to_string(),
            meta_tile: HatMetaTileType::H,
        })
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_visuals_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_system(ui_system);
    }
}
fn configure_visuals_system(mut egui_settings: ResMut<EguiSettings>) {
    egui_settings.scale_factor = 1.5;
}

fn ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UIState>,
    mut life_config: ResMut<LifeConfig>,
    mut tree_config: ResMut<TreeConfig>,
    mut evt1: EventWriter<AddNoiseEvent>,
    mut evt2: EventWriter<RemoveNoiseEvent>,
    mut step_timer: ResMut<StepTimer>,
) {
    contexts.ctx_mut().set_visuals(egui::Visuals::light());

    egui::SidePanel::left("side_panel")
        // .default_width(500.0)
        .exact_width(150.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Settings");

            let running_label = if life_config.running { "Pause" } else { "Play" };
            if ui.button(running_label).clicked() {
                life_config.running = !life_config.running;
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

            ui.horizontal(|ui| {
                if ui.button("Fill").clicked() {
                    evt1.send(AddNoiseEvent { fraction: 1.01 });
                };
                if ui.button("Clear").clicked() {
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
                ui.label("Stroke width:");
                if ui
                    .text_edit_singleline(&mut ui_state.stroke_width)
                    .changed()
                {
                    life_config.stroke_width = ui_state.stroke_width.parse::<usize>().unwrap_or(1)
                }
            });

            ui.separator();

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
                    // life_config.survival = ui_state
                    //     .survival
                    //     .chars()
                    //     .filter_map(|c| c.to_digit(10))
                    //     .collect();
                }
            });

            ui.separator();

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
            })
            // ui.add(egui::widgets::Image::new(
            //     egui_texture_handle.id(),
            //     egui_texture_handle.size_vec2(),
            // ));

            // ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
        });
    // egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
    //     ui.label("world");
    // });
}
