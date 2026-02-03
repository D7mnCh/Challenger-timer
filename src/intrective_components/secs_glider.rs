use egui::Ui;
use crate::Data;
use crate::Session;
use std::time::*;

#[derive(Default)]
pub struct SecsGlider{
    work_mins: u64,
    rest_mins: u64
}
impl SecsGlider {
    fn secs_converter (&mut self, data: &mut Data) {
        data.work_secs = self.work_mins * 60;
        data.rest_secs = self.rest_mins * 60;
    }
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        // Work secs dragger
        ui.add(egui::DragValue::new(&mut self.work_mins)
            .speed(1)
            .prefix("Work: ")
        );

        // Rest secs dragger
        ui.add(egui::DragValue::new(&mut self.rest_mins)
            .speed(1)
            .prefix("Rest: ")
        );
        self.secs_converter(data);
    }
}
