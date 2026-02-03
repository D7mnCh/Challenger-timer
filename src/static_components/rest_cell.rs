use egui::Ui;

use crate::Data;
use crate::Session;
#[derive(Default)]
// for this cell, if it work session it will increase by a little. if it rest sission, the secs
// from ths struct will get converted to switch block
pub struct RestCell{
    secs: u64,
    mins: u64,
    hours: u64,
}
impl RestCell {
    fn update_time(&mut self, data: &mut Data){
        self.mins = self.secs / 60;
        self.hours= self.secs / (60*60);
    }
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data){
        self.update_time(data);

        if data.pause == false {
            if let Session::Rest = data.session {
                self.secs += data.instant.elapsed().as_secs();
            }
        }

        let degital_clock = format!("Rest: {:02}:{:02}:{:02}", self.hours % 24, self.mins % 60, self.secs % 60);
        let degital_clock = degital_clock.as_str();
        ui.label(degital_clock);
    }
}
