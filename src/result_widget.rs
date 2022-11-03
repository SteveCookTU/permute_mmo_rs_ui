use eframe::egui;
use eframe::egui::style::Margin;
use eframe::egui::{Response, RichText, Rounding, Stroke, Ui, Widget};
use eframe::epaint::Shadow;

#[derive(Clone)]
pub struct PermuteResultWidget<'a> {
    result: &'a crate::PermuteResult,
}

impl<'a> PermuteResultWidget<'a> {
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
        })
        .response
    }
}
