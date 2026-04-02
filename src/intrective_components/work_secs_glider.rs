use crate::Data;
use crate::Session;

#[derive(Default)]
pub struct WorkSecsGlider {
    work_mins: u64,
}
impl WorkSecsGlider {
    fn secs_converter(&mut self, data: &mut Data) {
        // reset came from here at start
        data.work_secs = self.work_mins * 60;
    }
    fn get_user_saved_input(&mut self, data: &mut Data) {
        self.work_mins = data.work_secs / 60;
    }

    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        let range = 1.0..120.0;
        self.get_user_saved_input(data);
        ui.add_sized(
            [20., 20.],
            egui::DragValue::new(&mut self.work_mins)
                .speed(0.1)
                .prefix("Work: ")
                .range(1.0..=120.0),
        );
        self.secs_converter(data);
    }
}
