use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::{GitHubApi, PullRequest, Table, DataOrEmpty};
use crate::github::Runs;

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum State {
    Welcome,
    Pulls,
    Runs,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    token: String,
    table: Table,
    state: State,

    #[serde(skip)]
    github: GitHubApi,

    #[serde(skip)]
    pulls: Arc<Mutex<BTreeMap<String, Vec<PullRequest>>>>,

    #[serde(skip)]
    runs: Arc<Mutex<BTreeMap<String, Vec<Runs>>>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            token: String::from("<GitHub PAT>"),
            table: Table::default(),
            state: State::Welcome,
            github: GitHubApi::default(),
            pulls: Arc::new(Mutex::new(BTreeMap::new())),
            runs: Arc::new(Mutex::new(BTreeMap::new())),
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
        let Self { token, table, state, github, pulls: _, runs: _ } = self;

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

            if ui.button("Fetch/Refresh").clicked() {
                let repos = [
                    "aap-andre-ytelser", "aap-api", "aap-bot", "aap-devtools", "aap-inntekt",
                    "aap-libs", "aap-meldeplikt", "aap-oppgavestyring", "aap-personopplysninger",
                    "aap-sink", "aap-sykepengedager", "aap-utbetaling", "aap-vedtak",
                ];

                for repo in repos {
                    let _pulls = self.pulls.clone();
                    github.pull_requests(token, &repo.to_string(), move |response: DataOrEmpty<Vec<PullRequest>>| {
                        let prs = match response {
                            DataOrEmpty::Data(prs) => prs,
                            DataOrEmpty::Empty{} => Vec::default(),
                        };

                        * _pulls.lock().unwrap().entry(repo.to_string()).or_default() = prs;
                    });
                }
            }

            if ui.button("Pull Requests").clicked() {
                *state = State::Pulls
            }

            if ui.button("Workflows").clicked() {
                *state = State::Runs
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Size, StripBuilder};

            ui.heading("Pull Requests");

            match state {
                State::Pulls => {
                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    table.table_ui(ui, &self.pulls.lock().unwrap().clone())
                                });
                            });
                        });
                }
                State::Runs => { ui.label("Workflow runs to be continued..."); }
                State::Welcome => { ui.label("Welcome you are!"); }
            };

            ui.separator();

            ui.label("Hello there!");
        });
    }
}
