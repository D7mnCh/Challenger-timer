use egui::*;
#[derive(Default)]
pub struct SwitchCell;
impl SwitchCell {
    pub fn display(&self, ui: &mut Ui,/*sec: &i32*/) {
        ui.put(
            Rect::from_min_max(pos2(0., 0.), pos2(100., 100.)),
            Label::new(RichText::new("20::00:00").background_color(Color32::DARK_GRAY)),
        );
        //println!("{}");
    }
}
