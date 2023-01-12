use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, Mutex};

use eframe::epaint::Color32;
use egui::TextFormat;
use ehttp::Response;

use crate::github::github_client::{GitHubApi, Pulls, Runs, Teams};
use crate::github::pulls::PullRequest;
use crate::github::runs::WorkflowRuns;
use crate::github::teams::Team;
use crate::github::workflows::Workflow;
use crate::ui::table::Table;

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            token,
            show_token,
            pr_table,
            run_table,
            state,
            repositories,
            new_repo,
            team,
            teams: _,
            teams_responses: _,
            github,
            pulls: _,
            workflows: _,
            runs: _,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Personal Access Token:");
                ui.add(egui::TextEdit::singleline(token).password(!show_token.clone()));

                if ui.add(egui::SelectableLabel::new(show_token.clone(), "👁"))
                    .on_hover_text("Show/hide token")
                    .clicked() { *show_token = !show_token.clone(); };
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("GitHub Status");

            ui.separator();
            ui.label("Overview of pull requests");

            if ui.button("Pull Requests").clicked() {
                *state = State::Pulls
            }

            ui.separator();
            ui.label("Latest runs in GitHub Actions");

            if ui.button("Workflows").clicked() {
                *state = State::Runs
            }

            ui.separator();
            ui.label("Select repositories");

            if ui.button("Repositories").clicked() {
                *state = State::Repositories
            }

            ui.separator();
            ui.label("Show teams");

            if ui.button("Teams").clicked() {
                *state = State::Teams
            }

            ui.separator();
            ui.label("Fetch data from GitHub");

            if ui.button("Refresh").clicked() {
                match state {
                    State::Pulls => {
                        for repo in repositories.clone().into_iter() {
                            let _pulls = self.pulls.clone();
                            github.pull_requests(token, &repo.to_string(), move |response: Vec<PullRequest>| {
                                *_pulls.lock().unwrap().entry(repo).or_default() = response;
                            });
                        }
                    }
                    State::Runs => {
                        for repo in repositories.clone().into_iter() {
                            let _runs = self.runs.clone();
                            github.runs(token, &repo.to_string(), move |response: WorkflowRuns| {
                                *_runs.lock().unwrap().entry(repo).or_insert(WorkflowRuns::default()) = response;
                            });
                        }
                        // for repo in repositories.clone().into_iter() {
                        //     let _workflows = self.workflows.clone();
                        //     github.workflows(token, &repo.to_string(), move |response: Vec<Workflow>| {
                        //         *_workflows.lock().unwrap().entry(repo).or_default() = response;
                        //     });
                        // }
                    }
                    State::Teams => {
                        for i in 1..=3 {
                            let _teams_responses = self.teams_responses.clone();
                            let url = format!("https://api.github.com/orgs/navikt/teams?per_page=100&page={}", i);

                            github.teams(&url, token, move |teams_response| {
                                _teams_responses.lock().unwrap().push(teams_response.clone());
                            });
                        }

                        let responses = self.teams_responses.clone();
                        let teams = self.teams.clone();
                        *teams.lock().unwrap() = responses.lock().unwrap().clone().into_iter().flat_map(|res| {
                            match serde_json::from_slice::<Vec<Team>>(&res.bytes) {
                                Ok(teams) => {
                                    println!("parsed {} teams from_slice", &teams.len());
                                    teams.clone()
                                },
                                Err(e) => {
                                    println!("error while parsing teams from_slice: {:?} ", e);
                                    vec![]
                                }
                            }
                        }).collect::<Vec<_>>();

                        ctx.request_repaint();

                    }
                    _ => println!("Unsupported refresh")
                }
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
                                    pr_table.pull_requests_ui(ui, &self.pulls.lock().unwrap().clone())
                                });
                            });
                        });
                }
                State::Runs => {
                    ui.heading("Workflow Runs");

                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    let _runs = &self.runs.lock().unwrap().clone();
                                    let _workflows = &self.workflows.lock().unwrap().clone();
                                    run_table.workflow_runs_ui(ui, _runs)
                                });
                            });
                        });
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
                State::Teams => {
                    ui.heading("Teams");
                    ui.label(format!("Found {} teams in org/navikt", self.teams.lock().unwrap().clone().len()));

                    let show_text = team.clone().map_or(String::from("Not selected"), |map| { map.name });

                    egui::ComboBox::from_label("team")
                        .selected_text(format!("{:?}", show_text))
                        .show_ui(ui, |ui| {
                            self.teams.lock().unwrap().clone().into_iter().for_each(|t| {
                                ui.selectable_value(team, Some(t.clone()), &t.name);
                            });
                        });
                }
            };
        });
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            token: String::from("<GitHub PAT>"),
            show_token: false,
            pr_table: Table::default(),
            run_table: Table::default(),
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
            team: None,
            teams: Arc::new(Mutex::new(vec![])),
            teams_responses: Arc::new(Mutex::new(vec![])),
            github: GitHubApi::default(),
            pulls: Arc::new(Mutex::new(BTreeMap::new())),
            workflows: Arc::new(Mutex::new(BTreeMap::new())),
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

#[derive(PartialEq)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum State {
    Repositories,
    Teams,
    Pulls,
    Runs,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    token: String,
    show_token: bool,
    pr_table: Table,
    run_table: Table,
    state: State,
    repositories: HashSet<String>,
    new_repo: String,
    team: Option<Team>,

    #[serde(skip)]
    teams: Arc<Mutex<Vec<Team>>>,

    #[serde(skip)]
    teams_responses: Arc<Mutex<Vec<Response>>>,

    #[serde(skip)]
    github: GitHubApi,

    #[serde(skip)]
    pulls: Arc<Mutex<BTreeMap<String, Vec<PullRequest>>>>,

    #[serde(skip)]
    workflows: Arc<Mutex<BTreeMap<String, Vec<Workflow>>>>,

    #[serde(skip)]
    runs: Arc<Mutex<BTreeMap<String, WorkflowRuns>>>,
}