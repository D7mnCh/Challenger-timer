use egui::*;

use crate::Data;

#[derive(Default)]
pub struct WorkCell;
impl WorkCell {
    pub fn display(&self, ui: &mut Ui, data: &mut Data) {
        let degital_clock = format!("{:02}:{:02}:{:02}", data.hours % 24, data.mins % 60, data.secs % 60);
        let degital_clock = degital_clock.as_str();

        ui.put(
            Rect::from_min_max(pos2(0., 0.), pos2(100., 100.)),
            Label::new(RichText::new(degital_clock).background_color(Color32::DARK_GRAY)),
        );
    }
}
