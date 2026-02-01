use egui::*;

use crate::Data;
#[derive(Default)]
pub struct SwitchCell;
impl SwitchCell {
    pub fn display(&self, ui: &mut Ui, data: &mut Data){
        let secs -= *data.instant; 
        let mins = *data.mins;
        let hours = *data.hours;

        let degital_clock = format!("{:02}:{:02}:{:02}", hours % 24, mins % 60, secs % 60);
        ui.label(degital_clock);
    }
}
