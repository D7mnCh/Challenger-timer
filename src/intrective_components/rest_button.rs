use egui::Ui;
use crate::Data;
use crate::Session;
use std::time::*;

#[derive(Default)]
pub struct RestButton;
impl RestButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui.button("Swtich to rest session").clicked() {
            //data.reset = true;
            data.new_user_input = true;
            data.pause = true;
            data.session = Session::Rest;
        }
    }
}
