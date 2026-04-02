use crate::data::Data;

#[derive(Default)]
pub struct ResetTotals;
impl ResetTotals {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if ui.button("Reset totals").clicked() {
            //data.reset_with_new_user_input = true;
            data.reset_totals = true;
            data.pause = true;
        }
    }
}
