use egui::Ui;

#[derive(Default)]
pub struct PauseButton;
impl PauseButton {
    pub fn display(&self, ui: &mut Ui, secs: &mut u64, mins: &u64, hors: &u64) {
        if ui.button("Pause").clicked() {
            //...
        }
    }
}

