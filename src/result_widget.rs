use eframe::egui;
use eframe::egui::style::Margin;
use eframe::egui::{Response, RichText, Rounding, Stroke, Ui, Widget};
use eframe::epaint::Shadow;
#[cfg(feature = "sysbot")]
use sysbot_rs::types::PokeData;
#[cfg(feature = "sysbot")]
use sysbot_rs::SysBotClient;

#[derive(Clone)]
pub struct PermuteResultWidget<'a> {
    result: &'a crate::PermuteResult,
    #[cfg(feature = "sysbot")]
    client: &'a Option<SysBotClient>,
}

impl<'a> PermuteResultWidget<'a> {
    #[cfg(feature = "sysbot")]
    pub fn new(result: &'a crate::PermuteResult, client: &'a Option<SysBotClient>) -> Self {
        Self { result, client }
    }

    #[cfg(not(feature = "sysbot"))]
    pub fn new(result: &'a crate::PermuteResult) -> Self {
        Self { result }
    }
}

impl Widget for PermuteResultWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Frame {
            inner_margin: Margin::same(10.0),
            outer_margin: Margin::same(5.0),
            rounding: Rounding::same(5.0),
            shadow: Shadow::default(),
            fill: ui.visuals().faint_bg_color,
            stroke: Stroke::default(),
        }
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new(&self.result.name).size(20.0).underline());
            });
            ui.add_space(5.0);
            ui.label(&self.result.seed);
            ui.add_space(5.0);
            egui::Grid::new(&self.result.name).show(ui, |ui| {
                for line in &self.result.lines {
                    ui.label(&line.path);
                    ui.vertical_centered(|ui| {
                        ui.label(">>>");
                    });
                    ui.label(line.wave_indicator.trim());
                    ui.label(line.spawn.trim());
                    ui.vertical_centered(|ui| {
                        ui.label("=");
                    });
                    ui.label(&line.name);
                    ui.label(line.gender);
                    ui.label(&line.shiny);
                    ui.label(&line.ivs);
                    ui.label(line.nature);
                    ui.label(line.not_alpha);
                    ui.label(&line.extras);
                    ui.end_row();
                }
            });
            #[cfg(feature = "sysbot")]
            {
                if self.result.x >= 0.0 && self.result.y >= 0.0 && self.result.z >= 0.0 {
                    ui.add_space(5.0);
                    ui.vertical_centered(|ui| {
                        if ui.button("Teleport").clicked() {
                            if let Some(client) = self.client.as_ref() {
                                teleport_player(
                                    client,
                                    self.result.x,
                                    self.result.y,
                                    self.result.z,
                                );
                            }
                        }
                    });
                }
            }
        })
        .response
    }
}

#[cfg(feature = "sysbot")]
fn teleport_player(client: &SysBotClient, x: f32, y: f32, z: f32) {
    let data = [
        x.round().to_le_bytes(),
        (y + 15.0).round().to_le_bytes(),
        z.round().to_le_bytes(),
    ]
    .concat();
    client
        .pointer_poke(
            &[0x42F18E8, 0x88, 0x90, 0x1F0, 0x18, 0x80, 0x90],
            PokeData::new(data),
        )
        .unwrap_or_default()
}
