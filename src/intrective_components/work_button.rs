use crate::Data;
use crate::Session;

#[derive(Default)]
pub struct WorkButton;
impl WorkButton {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if ui.button("Swtich to work session").clicked() {
            //data.reset = true;
            data.reset_with_new_user_input = true;
            data.pause = true;
            data.session = Session::Work;
        }
    }
}
