use egui::*;
use crate::Data;
use crate::Session;

#[derive(Default)]
pub struct WorkCell{
    secs: u64,
    mins: u64,
    hours: u64,
}
impl WorkCell {
    fn update_time(&mut self, data: &mut Data){
        self.mins = self.secs / 60;
        self.hours= self.secs / (60*60);
    }
    pub fn display(&mut self, ui: &mut Ui, data: &mut Data) {
        self.update_time(data);
        if data.pause == false {
            if let Session::Work = data.session {
                self.secs += data.instant.elapsed().as_secs();
            }
        }
        let degital_clock = format!("Work: {:02}:{:02}:{:02}", self.hours % 24, self.mins % 60, self.secs % 60);
        let degital_clock = degital_clock.as_str();
        ui.label(degital_clock);

      //  ui.put(
      //      Rect::from_min_max(pos2(0., 0.), pos2(100., 100.)),
      //      Label::new(RichText::new(degital_clock).background_color(Color32::DARK_GRAY)),
      //  );
    }
}
