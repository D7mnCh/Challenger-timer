use egui::Ui;

use crate::Data;
use std::time::*;

#[derive(Default)]
pub struct PauseButton;
impl PauseButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui.button("Pause").clicked() {
            data.pause = ! data.pause;
        }
        if data.pause {
            data.instant = Instant::now();
        }
    }
}
