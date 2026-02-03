use egui::Ui;

use crate::Data;
use crate::Session;
use std::time::*;

#[derive(Default)]
pub struct WorkButton;
impl WorkButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui.button("Swtich to work session").clicked() {
            //data.reset = true;
            data.new_user_input = true;
            data.pause = true;
            data.session = Session::Work;
        }
    }
}
