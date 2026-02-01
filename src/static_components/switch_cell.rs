// this should be in the work cell because it will increase over time
// the switch_cell will always be dencreasing in both rest and work sessions
use egui::*;
#[derive(Default)]
pub struct SwitchCell;
impl SwitchCell {
    pub fn display(&self, ui: &mut Ui, secs: &mut u64, mins: &u64, hors: &u64) {
        let degital_clock = format!("{:02}:{:02}:{:02}", *hors % 24, *mins % 60, *secs % 60);
        let degital_clock = degital_clock.as_str();

        ui.put(
            Rect::from_min_max(pos2(0., 0.), pos2(100., 100.)),
            Label::new(RichText::new(degital_clock).background_color(Color32::DARK_GRAY)),
        );
    }
}
