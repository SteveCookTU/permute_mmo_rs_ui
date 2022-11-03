use crate::spawner::Spawner;
use crate::{PermuteResultWidget, NATURES_EN, SPECIES_EN};
use eframe::egui;
#[cfg(feature = "sysbot")]
use eframe::egui::panel::Side;
#[cfg(target_arch = "wasm32")]
use eframe::egui::RichText;
use eframe::egui::{DroppedFile, Visuals};
use permute_mmo_rs::generation::EntityResult;
use permute_mmo_rs::permutation::{Advance, PermuteMeta};
use permute_mmo_rs::permuter;
use permute_mmo_rs::structure::{
    MassOutbreakSet8a, MassiveOutbreakArea8a, MassiveOutbreakSet8a, MassiveOutbreakSpawnerStatus,
};
use permute_mmo_rs::util::area_util::AREA_TABLE;
use permute_mmo_rs::util::{area_util, SpawnInfo};
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;
use std::rc::Rc;
#[cfg(feature = "sysbot")]
use sysbot_rs::SysBotClient;

pub struct PermuteMMO {
    #[cfg(feature = "sysbot")]
    ip: String,
    #[cfg(feature = "sysbot")]
    port: u16,
    #[cfg(feature = "sysbot")]
    status: &'static str,
    #[cfg(feature = "sysbot")]
    client: Option<SysBotClient>,
    results: Vec<PermuteResult>,
}

pub struct PermuteResult {
    pub name: String,
    pub seed: String,
    pub lines: Vec<ResultLine>,
}

pub struct ResultLine {
    pub path: String,
    pub wave_indicator: String,
    pub spawn: String,
    pub shiny: String,
    pub name: String,
    pub gender: &'static str,
    pub not_alpha: &'static str,
    pub ivs: String,
    pub extras: String,
    pub nature: &'static str,
}

impl PermuteMMO {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals::dark());
        Self {
            #[cfg(feature = "sysbot")]
            ip: String::new(),
            #[cfg(feature = "sysbot")]
            port: 6000,
            #[cfg(feature = "sysbot")]
            status: "",
            #[cfg(feature = "sysbot")]
            client: None,
            results: vec![],
        }
    }

    pub fn permute_massive_mass_outbreak(&mut self, block: MassiveOutbreakSet8a) {
        let criteria = |entity: &EntityResult, _: &[Advance]| -> bool { entity.is_shiny };
        for i in 0..MassiveOutbreakSet8a::AREA_COUNT {
            let area = block[i];
            let area_name = if let Some(area_name) = area_util::AREA_TABLE.get(&area.area_hash) {
                area_name
            } else {
                AREA_TABLE.get(&0).unwrap()
            };
            if area.is_active {
                for j in 0..MassiveOutbreakArea8a::SPAWNER_COUNT {
                    let spawner = area[j];
                    if spawner.status() == MassiveOutbreakSpawnerStatus::None {
                        continue;
                    }

                    let seed = spawner.group_seed;
                    let spawn: Rc<RefCell<SpawnInfo>> = spawner.into();

                    let result = permuter::permute(spawn.clone(), seed, 15, Some(criteria));
                    if !result.has_results() {
                        continue;
                    }

                    let mut permute_result = PermuteResult {
                        name: format!(
                            "{area_name} MMO Spawner {} at ({:.1}, {:.1}. {}) displays {}",
                            j + 1,
                            spawner.x,
                            spawner.y,
                            spawner.z,
                            SPECIES_EN[spawner.display_species as usize]
                        ),
                        seed: format!("Seed: 0x{:0>16X}", seed),
                        lines: vec![],
                    };

                    add_results(&mut permute_result, result);

                    self.results.push(permute_result)
                }
            }
        }
    }

    pub fn permute_mass_outbreak(&mut self, block: MassOutbreakSet8a) {
        let criteria = |entity: &EntityResult, _: &[Advance]| -> bool { entity.is_shiny };
        for i in 0..MassiveOutbreakSet8a::AREA_COUNT {
            let spawner = block[i];
            let area_name = if let Some(area_name) = area_util::AREA_TABLE.get(&spawner.area_hash) {
                area_name
            } else {
                AREA_TABLE.get(&0).unwrap()
            };
            if spawner.has_outbreak() {
                let seed = spawner.group_seed;
                let spawn: Rc<RefCell<SpawnInfo>> = spawner.into();

                let result = permuter::permute(spawn.clone(), seed, 15, Some(criteria));
                if !result.has_results() {
                    continue;
                }

                let mut permute_result = PermuteResult {
                    name: format!(
                        "{area_name} Outbreak Spawner at ({:.1}, {:.1}. {}) displays {}",
                        spawner.x,
                        spawner.y,
                        spawner.z,
                        SPECIES_EN[spawner.display_species as usize]
                    ),
                    seed: format!("Seed: 0x{:0>16X}", seed),
                    lines: vec![],
                };

                add_results(&mut permute_result, result);

                self.results.push(permute_result)
            }
        }
    }

    #[cfg(feature = "sysbot")]
    pub fn permute_massive_mass_outbreak_sysbot(&mut self) {
        if let Some(client) = self.client.as_ref() {
            if let Ok(data) = client.pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x1B0], 0x3980) {
                let data = &data[..(data.len() - 1)];
                let block: MassiveOutbreakSet8a = data.into();
                self.permute_massive_mass_outbreak(block);
            } else {
                self.status = "Failed to get MMO data from the switch";
            }
        }
    }

    #[cfg(feature = "sysbot")]
    pub fn permute_mass_outbreak_sysbot(&mut self) {
        if let Some(client) = self.client.as_ref() {
            if let Ok(data) = client.pointer_peek(&[0x42BA6B0, 0x2B0, 0x58, 0x18, 0x20], 0x190) {
                let data = &data[..(data.len() - 1)];
                let block: MassOutbreakSet8a = data.into();
                self.permute_mass_outbreak(block);
            } else {
                self.status = "Failed to get MMO data from the switch";
            }
        }
    }

    pub fn permute_single(
        &mut self,
        spawn: impl Into<Rc<RefCell<SpawnInfo>>>,
        seed: u64,
        species: u16,
    ) {
        let criteria = |entity: &EntityResult, _: &[Advance]| -> bool { entity.is_shiny };
        let result = permuter::permute(spawn.into(), seed, 15, Some(criteria));
        if result.has_results() {
            let mut permute_result = PermuteResult {
                name: format!("Single Spawner displays {}", SPECIES_EN[species as usize]),
                seed: format!("Seed: 0x{:0>16X}", seed),
                lines: vec![],
            };

            add_results(&mut permute_result, result);

            self.results.push(permute_result)
        }
    }
}

fn add_results(permute_result: &mut PermuteResult, result: PermuteMeta) {
    for (i, inner) in result.results.iter().enumerate() {
        let parent = result.find_nearest_parent_advance_result(i, &inner.advances);
        let is_action_multi_result = result.is_action_multi_result(i, &inner.advances);
        let has_child_chain = result.has_child_chain(i, &inner.advances);
        let mut extras = String::new();
        if parent.is_some() || has_child_chain {
            extras = format!("{} ~~ Chain result!", extras);
        }
        if is_action_multi_result {
            extras = format!("{} ~~ Spawns multiple results!", extras);
        }
        extras = format!("{}{}", extras, inner.get_feasibility(&inner.advances));
        permute_result.lines.push(ResultLine {
            path: inner.get_steps(parent).replace('|', " | "),
            wave_indicator: inner.get_wave_indicator().trim().to_string(),
            spawn: format!("Spawn {}", inner.entity.index),
            shiny: inner.entity.get_shiny_str(),
            name: {
                let alpha = if inner.entity.is_alpha { "α-" } else { "" };
                format!("{alpha}{}", inner.entity.slot.name)
            },
            gender: match inner.entity.gender {
                2 => "",
                1 => "(F)",
                _ => "(M)",
            },
            not_alpha: if !inner.entity.is_alpha {
                "-- NOT ALPHA"
            } else {
                ""
            },
            ivs: inner
                .entity
                .ivs
                .iter()
                .map(|iv| format!("{:0>2}", iv))
                .collect::<Vec<_>>()
                .join(" / "),
            extras,
            nature: NATURES_EN[inner.entity.nature as usize],
        })
    }
}

impl eframe::App for PermuteMMO {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(10.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("Drag and Drop Files Into This Window")
                            .size(50.0)
                            .underline(),
                    );
                });
                ui.add_space(10.0);
            });
        }

        #[cfg(feature = "sysbot")]
        {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(2.5);
                ui.horizontal(|ui| {
                    if ui.button("MMO").clicked() {
                        self.results.clear();
                        self.permute_massive_mass_outbreak_sysbot();
                    }
                    if ui.button("Outbreak").clicked() {
                        self.results.clear();
                        self.permute_mass_outbreak_sysbot();
                    }
                });
                ui.add_space(2.5);
            });
            egui::SidePanel::new(Side::Left, "left-panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("IP: ");
                    ui.text_edit_singleline(&mut self.ip);
                });
                ui.horizontal(|ui| {
                    ui.label("Port: ");
                    ui.add(egui::DragValue::new(&mut self.port));
                });

                if ui
                    .add_enabled(self.client.is_none(), egui::Button::new("Connect"))
                    .clicked()
                {
                    if let Ok(client) = SysBotClient::connect(self.ip.as_str(), self.port) {
                        self.client = Some(client);
                        self.status = "Connected to switch";
                    } else {
                        self.status = "Failed to connect to switch";
                    }
                }

                if ui
                    .add_enabled(self.client.is_some(), egui::Button::new("Disconnect"))
                    .clicked()
                {
                    self.client = None;
                    self.status = "Disconnected from switch";
                }

                ui.label(self.status);
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for result in self.results.iter() {
                    ui.add(PermuteResultWidget::new(result));
                }
            });
        });

        if !ctx.input().raw.dropped_files.is_empty() {
            let files: Vec<DroppedFile> = ctx.input().raw.dropped_files.clone();
            if let Some(file) = files.first() {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(path) = file.path.as_ref() {
                    if let Ok(mut file) = File::open(path) {
                        if let Some(extension) = path.extension() {
                            if extension == "json" || extension == "txt" {
                                let mut spawner_str = String::new();
                                if let Ok(_) = file.read_to_string(&mut spawner_str) {
                                    let spawner = serde_json::from_str::<Spawner>(&spawner_str)
                                        .unwrap_or_default();
                                    let seed = if spawner.seed.starts_with("0x") {
                                        u64::from_str_radix(&spawner.seed[2..], 16)
                                            .unwrap_or_default()
                                    } else {
                                        u64::from_str_radix(&spawner.seed, 16).unwrap_or_default()
                                    };
                                    let species = spawner.species;
                                    self.permute_single(spawner, seed, species);
                                }
                            } else {
                                let mut bytes = Vec::with_capacity(
                                    MassiveOutbreakSet8a::SIZE + MassOutbreakSet8a::SIZE,
                                );
                                if let Ok(len) = file.read_to_end(&mut bytes) {
                                    match len {
                                        i if i
                                            >= MassiveOutbreakSet8a::SIZE
                                                + MassOutbreakSet8a::SIZE =>
                                        {
                                            self.results.clear();
                                            let mass_data = &bytes[..MassOutbreakSet8a::SIZE];
                                            let massive_data = &bytes[MassiveOutbreakSet8a::SIZE..];
                                            self.permute_massive_mass_outbreak(massive_data.into());
                                            self.permute_mass_outbreak(mass_data.into());
                                        }
                                        i if i >= MassiveOutbreakSet8a::SIZE => {
                                            self.results.clear();
                                            self.permute_massive_mass_outbreak(
                                                bytes.as_slice().into(),
                                            );
                                        }
                                        i if i >= MassOutbreakSet8a::SIZE => {
                                            self.results.clear();
                                            self.permute_mass_outbreak(bytes.as_slice().into());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }

                #[cfg(target_arch = "wasm32")]
                if let Some(bytes) = file.bytes.as_ref() {
                    let bytes = bytes.to_vec();
                    if file.name.ends_with(".json") || file.name.ends_with(".txt") {
                        let spawner_str = String::from_utf8(bytes).unwrap_or_default();
                        if !spawner_str.is_empty() {
                            let spawner =
                                serde_json::from_str::<Spawner>(&spawner_str).unwrap_or_default();
                            let seed = if spawner.seed.starts_with("0x") {
                                u64::from_str_radix(&spawner.seed[2..], 16).unwrap_or_default()
                            } else {
                                u64::from_str_radix(&spawner.seed, 16).unwrap_or_default()
                            };
                            let species = spawner.species;
                            self.permute_single(spawner, seed, species);
                        }
                    } else {
                        match bytes.len() {
                            i if i >= MassiveOutbreakSet8a::SIZE + MassOutbreakSet8a::SIZE => {
                                self.results.clear();
                                let mass_data = &bytes[..MassOutbreakSet8a::SIZE];
                                let massive_data = &bytes[MassiveOutbreakSet8a::SIZE..];
                                self.permute_massive_mass_outbreak(massive_data.into());
                                self.permute_mass_outbreak(mass_data.into());
                            }
                            i if i >= MassiveOutbreakSet8a::SIZE => {
                                self.results.clear();
                                self.permute_massive_mass_outbreak(bytes.as_slice().into());
                            }
                            i if i >= MassOutbreakSet8a::SIZE => {
                                self.results.clear();
                                self.permute_mass_outbreak(bytes.as_slice().into());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
