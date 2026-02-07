use crate::data::*;

#[derive(Default, Debug)]
pub struct SwitchCell {
    // i should give them initial values from data
    work_secs: u64,
    rest_secs: u64,
    mins: u64,
    hours: u64,
}
impl SwitchCell {
    fn get_new_user_input(&mut self, data: &mut Data) {
        self.work_secs = data.work_secs;
        self.rest_secs = data.rest_secs;
    }
    fn update_time(&mut self, data: &mut Data) {
        match data.session {
            Session::Work => {
                self.mins = self.work_secs / 60;
                self.hours = self.work_secs / (60 * 60);
            }
            Session::Rest => {
                self.mins = self.rest_secs / 60;
                self.hours = self.rest_secs / (60 * 60);
            }
        }
    }
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        self.update_time(data);

        if data.reset_with_new_user_input == true {
            self.get_new_user_input(data);
            data.reset_with_new_user_input = false;
        }
        if data.pause == false {
            match data.session {
                Session::Work => {
                    if self.work_secs > 0 {
                        self.work_secs -= data.instant.elapsed().as_secs();
                    } else {
                        data.pause = true;

                        data.command = Command::Sound(SoundCommand::Play);
                        data.command.process_with(&mut data.child_process);
                    }
                }
                Session::Rest => {
                    if self.rest_secs > 0 {
                        self.rest_secs -= data.instant.elapsed().as_secs();
                    } else {
                        data.pause = true;

                        data.command = Command::Sound(SoundCommand::Play);
                        data.command.process_with(&mut data.child_process);
                    }
                }
            }
        }
        match data.session {
            Session::Work => {
                let degital_clock = format!(
                    "Work session: {:02}:{:02}:{:02}",
                    self.hours % 24,
                    self.mins % 60,
                    self.work_secs % 60
                );
                ui.label(egui::RichText::new(degital_clock).color(egui::Color32::RED).code());
            }
            Session::Rest => {
                let degital_clock = format!(
                    "Rest session: {:02}:{:02}:{:02}",
                    self.hours % 24,
                    self.mins % 60,
                    self.rest_secs % 60
                );
                ui.label(egui::RichText::new(degital_clock).color(egui::Color32::GREEN).code());
            }
        }
    }
}
