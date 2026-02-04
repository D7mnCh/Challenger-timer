pub mod pause_button;
pub mod rest_button;
pub mod rest_secs_glider;
pub mod work_button;
pub mod work_secs_glider;

// crate begin with the this IntrComp directory ?, i think cause have the same name ?
use crate::PauseButton;
use crate::RestButton;
use crate::RestSecsGlider;
use crate::WorkButton;
use crate::WorkSecsGlider;

// crate begin from root here ? cause this is true ?
use crate::Data;

#[derive(Default)]
pub struct IntrComp {
    pub work_button: WorkButton,
    pub rest_button: RestButton,
    pub pause_button: PauseButton,
    pub work_secs_glider: WorkSecsGlider,
    pub rest_secs_glider: RestSecsGlider,
}
impl IntrComp {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        /*
        ui.vertical(|ui| {
            //ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min),|ui| {
                self.pause_button.display(ui,data);
                self.rest_button.display(ui,data);
                self.work_button.display(ui,data);
            });
            //ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                self.rest_secs_glider.display(ui, data);
                self.work_secs_glider.display(ui, data);
            });
        });
        */
    }
}
