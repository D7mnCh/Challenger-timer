use egui::*;
use std::time::*;
use crate::Data;

// i think i will make 2 nested structs(swtich_cell_work,switch_cell_rest) inside SwtichCell one for work and one for rest and
// toggle between theme

#[derive(Default,Debug)]
pub struct SwitchCell{
    secs: u64,
    mins: u64,
    hours: u64,
}
impl SwitchCell {
    pub fn get_user_input(&mut self,data: &mut Data) {
        self.secs = data.secs;
    }
    pub fn update_time(&mut self, data: &mut Data){
        self.mins = self.secs / 60;
        self.hours= self.secs / (60*60);
    }
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data){
        self.update_time(data);
        if data.new_user_input == true{
            self.get_user_input(data);
        }
        if data.pause == false {
            self.secs -= data.instant.elapsed().as_secs();
        }
        let degital_clock = format!("Swtich: {:02}:{:02}:{:02}", self.hours % 24, self.mins % 60, self.secs % 60);
        ui.label(degital_clock);
    }
}
