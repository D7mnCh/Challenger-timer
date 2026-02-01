pub mod pause_button;
pub mod rest_button;
pub mod work_button;

use crate::PauseButton;

#[derive(Default)]
pub struct IntrComp {
    //work_button: WorkButton,
    //rest_button: RestButton,
    pub pause_button: PauseButton
}

impl IntrComp {
    pub fn display(&self, ui: &mut egui::Ui, secs: &mut u64, mins: &u64, hors: &u64) {
        self.pause_button.display(ui,secs,mins,hors);
        //self.work_cell.display(ui,secs,mins,hors);
        //self.rest_cell.display(ui,secs,mins,hors);
    }
}
