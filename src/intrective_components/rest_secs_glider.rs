use crate::Data;
use crate::Session;

#[derive(Default)]
pub struct RestSecsGlider {
    rest_mins: u64,
}
impl RestSecsGlider {
    fn secs_converter(&mut self, data: &mut Data) {
        data.rest_secs = self.rest_mins * 60;
    }

    fn get_user_saved_input(&mut self, data: &mut Data) {
        self.rest_mins = data.rest_secs / 60;
    }

    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        self.get_user_saved_input(data);

        ui.add_sized(
            [20., 20.],
            egui::DragValue::new(&mut self.rest_mins)
                .speed(0.1)
                .prefix("Rest: "),
        );

        self.secs_converter(data); // -> 90
    }
}
