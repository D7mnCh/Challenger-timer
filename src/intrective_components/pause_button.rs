use crate::Data;

#[derive(Default)]
pub struct PauseButton;
impl PauseButton {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if ui.button("Play/Pause").clicked() {
            data.pause = !data.pause;
        }
    }
}
