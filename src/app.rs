use std::sync::{Arc, Mutex};
use crate::Pulls;
use crate::pulls::{GitHubApi, PullRequest};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,

    #[serde(skip)]
    github: GitHubApi,

    #[serde(skip)]
    pull_requests: Arc<Mutex<Vec<PullRequest>>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: String::from("<GitHub PAT>"),
            github: GitHubApi::create(),
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
        let Self { label, github, pull_requests } = self;

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
                if ui.text_edit_singleline(label).lost_focus() {
                    github.update_token(label.to_owned());
                    println!("{}", &label);
                };
            });

            // github.repos().for_each(|repo: &&str| {
            //     if ui.button(repo.to_string()).clicked() {
            //         match github.pull_requests(repo) {
            //             Ok(res) => *pull_requests = res.pull_requests,
            //             Err(err) => println!("Error: {:?}", err)
            //         }
            //     }
            // });

            github.repos().for_each(|repo: &&str| {
                if ui.button(repo.to_string()).clicked() {
                    let prs = self.pull_requests.clone();
                    github.pull_requests(repo, move |pulls: Pulls| {
                        *prs.lock().unwrap() = pulls.pull_requests;
                    });
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pull Requests");

            let prs = self.pull_requests.clone();
            if prs.lock().unwrap().is_empty() {
                ui.label("All good!");
            } else {
                prs.lock().unwrap().clone().into_iter().for_each(|pr| {
                    ui.hyperlink(&pr.html_url);
                });
            }
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
