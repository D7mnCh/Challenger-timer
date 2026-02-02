use egui::Ui;

use crate::Data;
use std::time::*;

#[derive(Default)]
// this struct intrects with SwtichCell
pub struct WorkButton{
    secs: u64
}
impl WorkButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if data.new_user_input == true {
            self.secs = data.secs;
        }
        if ui.button("Swtich to work session").clicked() {
            data.secs = self.secs;
            // to intrect with SwitchCell to update his secs
            data.new_user_input = true;
        }
    println!("{data:#?}");
    }
}
