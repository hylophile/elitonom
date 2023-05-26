use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{
    life::{noise::AddNoiseEvent, LifeConfig, StepTimer},
    tree::{MetaTileType, TreeConfig},
};

pub struct UIPlugin;

#[derive(Resource)]
struct UIState {
    birth: String,
    survival: String,
    update_interval: String,
    levels: String,
    meta_tile: MetaTileType,
}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UIState {
            birth: "3".to_string(),
            survival: "23".to_string(),
            update_interval: "0.1".to_string(),
            levels: "5".to_string(),
            meta_tile: MetaTileType::H,
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
    mut evt: EventWriter<AddNoiseEvent>,
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

            if ui.button("Add Noise").clicked() {
                evt.send(AddNoiseEvent);
            }

            ui.horizontal(|ui| {
                ui.label("Birth:");
                let response = ui.text_edit_singleline(&mut ui_state.birth);
                if response.changed() {
                    life_config.birth = ui_state
                        .birth
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .collect();
                }
            });
            ui.horizontal(|ui| {
                ui.label("Survival:");
                let response = ui.text_edit_singleline(&mut ui_state.survival);
                if response.changed() {
                    life_config.survival = ui_state
                        .survival
                        .chars()
                        .filter_map(|c| c.to_digit(10))
                        .collect();
                }
            });
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

            ui.horizontal(|ui| {
                if ui
                    .radio_value(&mut ui_state.meta_tile, MetaTileType::H, "H")
                    .clicked()
                {
                    ui_state.meta_tile = MetaTileType::H;
                    tree_config.meta_tile = MetaTileType::H;
                };
                if ui
                    .radio_value(&mut ui_state.meta_tile, MetaTileType::T, "T")
                    .clicked()
                {
                    ui_state.meta_tile = MetaTileType::T;
                    tree_config.meta_tile = MetaTileType::T;
                };

                if ui
                    .radio_value(&mut ui_state.meta_tile, MetaTileType::P, "P")
                    .clicked()
                {
                    ui_state.meta_tile = MetaTileType::P;
                    tree_config.meta_tile = MetaTileType::P;
                };
                if ui
                    .radio_value(&mut ui_state.meta_tile, MetaTileType::F, "F")
                    .clicked()
                {
                    ui_state.meta_tile = MetaTileType::F;
                    tree_config.meta_tile = MetaTileType::F;
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
