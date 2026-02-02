use egui::Ui;

use crate::Data;
#[derive(Default)]
pub struct RestCell;
impl RestCell {
    pub fn display(&self, ui: &mut Ui, data: &mut Data){
        let degital_clock = format!("Rest: {:02}:{:02}:{:02}", data.hours % 24, data.mins % 60, data.secs % 60);
        let degital_clock = degital_clock.as_str();
        ui.label(degital_clock);
    }
}
