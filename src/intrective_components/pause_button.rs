use egui::Ui;

use std::time::*;
use crate::Data;


#[derive(Default)]
pub struct PauseButton{
    pause: bool
}
impl PauseButton {
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        if ui.button("Pause").clicked() {
            //data.pre_conf_secs = data.secs;
            self.pause = ! self.pause;
        }
        if self.pause {
            data.instant = Instant::now();
        }
    }
}
