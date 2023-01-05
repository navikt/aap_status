#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Table {
    striped: bool,
    resizable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            striped: true,
            resizable: true,
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
        }
    }
}

const NUM_MANUAL_ROWS: usize = 20;

impl Table {
    pub fn table_ui(&mut self, ui: &mut egui::Ui) {
        use egui_extras::{Column, TableBuilder};

        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).range(40.0..=300.0).resizable(true))
            .column(
                Column::initial(100.0)
                    .at_least(40.0)
                    .resizable(true)
                    .clip(true),
            )
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        if let Some(row_nr) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_nr, None);
        }

        table.header(20.0, |mut header| {
            header.col(|ui| { ui.strong("Row"); });
            header.col(|ui| { ui.strong("Expanding content"); });
            header.col(|ui| { ui.strong("Clipped text"); });
            header.col(|ui| { ui.strong("Content"); });
        })
            .body(|mut body| {
                for row_index in 0..NUM_MANUAL_ROWS {
                    let is_thick = thick_row(row_index);
                    let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        row.col(|ui| { ui.label(row_index.to_string()); });
                        row.col(|ui| { expanding_content(ui); });
                        row.col(|ui| { ui.label(long_text(row_index)); });
                        row.col(|ui| {
                            ui.style_mut().wrap = Some(false);
                            if is_thick { ui.heading("Extra thick row"); } else { ui.label("Normal row"); }
                        });
                    });
                }
            });
    }
}

fn expanding_content(ui: &mut egui::Ui) {
    let width = ui.available_width().clamp(20.0, 200.0);
    let height = ui.available_height();
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        (1.0, ui.visuals().text_color()),
    );
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}
