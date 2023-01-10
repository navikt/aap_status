use std::collections::BTreeMap;

use egui::Ui;

use crate::PullRequest;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Table {
    striped: bool,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            striped: false,
        }
    }
}

impl Table {
    pub fn pull_requests_ui(&mut self, ui: &mut Ui, pulls: &BTreeMap<String, Vec<PullRequest>>) {
        use egui_extras::{Column, TableBuilder};

        let table = TableBuilder::new(ui)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).at_least(50.0).resizable(true).clip(true))
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0);

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("ID"); });
            header.col(|ui| { ui.strong("Title"); });
            header.col(|ui| { ui.strong("Last Update"); });
            header.col(|ui| { ui.strong("Author"); });
        })
            .body(|mut body| {
                for (name, prs) in pulls.into_iter() {

                    if !prs.is_empty() {
                        body.row(40.0, |mut row| {
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(name); });
                            row.col(|ui| { ui.heading(""); });
                            row.col(|ui| { ui.heading(""); });
                        });
                    }

                    prs.into_iter().for_each(|pr| {
                        let _pr = pr.clone();
                        body.row(18.0, |mut row| {
                            row.col(|ui| { ui.label(format!("{}", &_pr.number)); });
                            row.col(|ui| { ui.hyperlink_to(&_pr.title.unwrap(), &_pr.html_url.unwrap()); });
                            row.col(|ui| { ui.label(&_pr.updated_at.unwrap()); });
                            row.col(|ui| { ui.label(&_pr.user.unwrap().login); });
                        });
                    });
                }
            });
    }
}
