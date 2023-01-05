use std::sync::{Arc, Mutex};

use crate::{Pulls, Table};
use crate::pulls::{GitHubApi, PullRequest};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    token: String,
    table: Table,

    #[serde(skip)]
    github: GitHubApi,

    #[serde(skip)]
    pull_requests: Arc<Mutex<Vec<PullRequest>>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            token: String::from("<GitHub PAT>"),
            table: Table::default(),
            github: GitHubApi::default(),
            pull_requests: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { token, table, github, pull_requests: _ } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Repositories");

            ui.horizontal(|ui| {
                ui.label("Token");
                ui.text_edit_singleline(token);
            });

            github.repos().for_each(|repo: &&str| {
                if ui.button(repo.to_string()).clicked() {
                    let prs = self.pull_requests.clone();
                    github.pull_requests(token, repo, move |pulls: Pulls| {
                        *prs.lock().unwrap() = pulls.pull_requests;
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Size, StripBuilder};

            ui.heading("Pull Requests");

            let prs = self.pull_requests.clone();
            if prs.lock().unwrap().is_empty() {
                ui.label("All good!");
            } else {
                StripBuilder::new(ui)
                    .size(Size::remainder().at_least(100.0))
                    .vertical(|mut strip| {
                        strip.cell(|ui| {
                            egui::ScrollArea::horizontal().show(ui, |ui| {
                                table.table_ui(ui, &prs.lock().unwrap().clone())
                            });
                        });
                    });
            }

            ui.separator();

            ui.label("Hello there!");
        });
    }
}
