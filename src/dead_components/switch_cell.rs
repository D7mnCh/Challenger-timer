#[derive(Default)]
pub struct SwitchCell;
impl SwitchCell {
    pub fn display(&self, ui: &mut egui::Ui) {

        ui.put(
            egui::Rect::from_min_max(egui::pos2(0., 0.), egui::pos2(100., 100.)),
            egui::Label::new(
                egui::RichText::new("20::00:00").background_color(egui::Color32::DARK_GRAY),
            ),
        );
    }
}
