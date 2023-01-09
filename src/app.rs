use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};
use eframe::epaint::Color32;
use egui::TextFormat;

use crate::{DataOrEmpty, GitHubApi, PullRequest, Table};
use crate::github::Runs;

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum State {
    Repositories,
    Pulls,
    Runs,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    token: String,
    show_token: bool,
    table: Table,
    state: State,
    repositories: HashSet<String>,

    // #[serde(skip)]
    new_repo: String,

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
            show_token: false,
            table: Table::default(),
            state: State::Repositories,
            repositories: HashSet::from([
                "aap-andre-ytelser".to_string(),
                "aap-api".to_string(),
                "aap-bot".to_string(),
                "aap-devtools".to_string(),
                "aap-inntekt".to_string(),
                "aap-libs".to_string(),
                "aap-meldeplikt".to_string(),
                "aap-oppgavestyring".to_string(),
                "aap-personopplysninger".to_string(),
                "aap-sink".to_string(),
                "aap-sykepengedager".to_string(),
                "aap-utbetaling".to_string(),
                "aap-vedtak".to_string(),
            ]),
            new_repo: String::from("<repo>"),
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
        let Self { token, show_token, table, state, repositories, new_repo, github, pulls: _, runs: _ } = self;

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
            ui.heading("GitHub Status");

            ui.horizontal_wrapped(|ui| {
                ui.label("GitHub PAT:");
                ui.add(egui::TextEdit::singleline(token).password(!show_token.clone()));

                if ui.add(egui::SelectableLabel::new(show_token.clone(), "👁"))
                    .on_hover_text("Show/hide token")
                    .clicked() { *show_token = !show_token.clone(); };
            });

            if ui.button("Fetch/Refresh").clicked() {
                for repo in repositories.clone().into_iter() {
                    let _pulls = self.pulls.clone();
                    github.pull_requests(token, &repo.to_string(), move |response: DataOrEmpty<Vec<PullRequest>>| {
                        let prs = match response {
                            DataOrEmpty::Data(prs) => prs,
                            DataOrEmpty::Empty {} => Vec::default(),
                        };

                        *_pulls.lock().unwrap().entry(repo.to_string()).or_default() = prs;
                    });
                }
            }

            if ui.button("Pull Requests").clicked() {
                *state = State::Pulls
            }

            if ui.button("Workflows").clicked() {
                *state = State::Runs
            }

            if ui.button("Repositories").clicked() {
                *state = State::Repositories
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            use egui_extras::{Size, StripBuilder};
            match state {
                State::Pulls => {
                    ui.heading("Pull Requests");

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
                State::Runs => {
                    ui.heading("Workflow Runs");

                    ui.label("Workflow runs to be continued...");
                }
                State::Repositories => {
                    ui.heading("Repositories");

                    ui.label(format!("Total: {}", repositories.len()));

                    repositories.clone().into_iter().for_each(|repo| {
                        ui.horizontal_wrapped(|ui| {
                            use egui::text::LayoutJob;
                            let mut job = LayoutJob::default();
                            let red_text = TextFormat {
                                color: Color32::from_rgb(255, 100, 100),
                                ..Default::default()
                            };
                            job.append("❌", 0.0, red_text);
                            if ui.button(job).clicked() {
                                repositories.remove(&repo);
                            };
                            ui.label(&repo);
                        });
                    });

                    ui.separator();

                    ui.label("Add repository");
                    ui.horizontal(|ui| {
                        if ui.text_edit_singleline(new_repo).ctx.input().key_pressed(egui::Key::Enter) {
                            repositories.insert(new_repo.to_string());
                        }

                        use egui::text::LayoutJob;
                        let mut job = LayoutJob::default();
                        let green_text = TextFormat {
                            color: Color32::from_rgb(100, 255, 146),
                            ..Default::default()
                        };

                        job.append("+", 0.0, green_text);

                        if ui.button(job).clicked() {
                            repositories.insert(new_repo.clone());
                        }
                    });
                }
            };
        });
    }
}
