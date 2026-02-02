pub mod pause_button;
pub mod rest_button;
pub mod work_button;

use crate::PauseButton;
use crate::RestButton;
use crate::WorkButton;

use crate::Data;

#[derive(Default)]
pub struct IntrComp {
    pub work_button: WorkButton,
    pub rest_button: RestButton,
    pub pause_button: PauseButton,
    //pub session: Session
}
impl IntrComp {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data ){
        self.pause_button.display(ui,data);
        self.rest_button.display(ui,data);
        self.work_button.display(ui,data);
    }
}
