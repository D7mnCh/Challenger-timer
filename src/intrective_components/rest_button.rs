use egui::Ui;

use crate::Data;
use std::time::*;

#[derive(Default)]
pub struct RestButton;
impl RestButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui.button("Swtich to rest session").clicked() {
            //..//
        }
    }
}
