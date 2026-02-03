pub mod pause_button;
pub mod rest_button;
pub mod work_button;
pub mod secs_glider;

use crate::PauseButton;
use crate::RestButton;
use crate::WorkButton;
use crate::secs_glider::*;

use crate::Data;

#[derive(Default)]
//i think buttons will not have fields, it just a button, but it can manipulate data
pub struct IntrComp {
    pub work_button: WorkButton,
    pub rest_button: RestButton,
    pub pause_button: PauseButton,
    pub secs_glider: SecsGlider
}
impl IntrComp {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data ){
        self.pause_button.display(ui,data);
        self.rest_button.display(ui,data);
        self.work_button.display(ui,data);
        self.secs_glider.display(ui,data);
    }
}
