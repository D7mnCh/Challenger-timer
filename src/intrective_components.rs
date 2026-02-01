pub mod pause_button;
pub mod rest_button;
pub mod work_button;

use crate::PauseButton;
use crate::Data;

#[derive(Default)]
pub struct IntrComp {
    //work_button: WorkButton,
    //rest_button: RestButton,
    pub pause_button: PauseButton,
}
impl IntrComp {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data ){
        self.pause_button.display(ui,data);
        //self.work_cell.display(ui,secs,mins,hors);
        //self.rest_cell.display(ui,secs,mins,hors);
    }
}
