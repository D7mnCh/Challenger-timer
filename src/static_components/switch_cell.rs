use crate::data::*;

// i need to fix something here when i suspend my laptop bug
// i think i'll fix the bug here
// it could be in the main mod

#[derive(Default, Debug)]
//NOTE for iniailization, use default trait
pub struct SwitchCell {
    work_secs: u64,
    rest_secs: u64,
    mins: u64,
    hours: u64,
    skip_pause_next: bool,
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

                if data.pause == false {
                    if self.work_secs > 0 {
                        self.work_secs -= data.instant.elapsed().as_secs();
                    }
                    if self.work_secs == 0 && self.skip_pause_next == true {
                        self.work_secs = 900;
                        self.skip_pause_next = false;

                        //play sound
                        data.command = Command::Sound(SoundCommand::Play);
                        data.command.process_with(&mut data.child_process);
                    }
                    if self.work_secs == 0 && self.skip_pause_next == false {
                        data.pause = true;

                        //play sound
                        data.command = Command::Sound(SoundCommand::Play);
                        data.command.process_with(&mut data.child_process);
                    }
                }
            }
            Session::Rest => {
                self.mins = self.rest_secs / 60;
                self.hours = self.rest_secs / (60 * 60);
                if data.pause == false {
                    if self.rest_secs > 0 {
                        self.rest_secs -= data.instant.elapsed().as_secs();
                    }
                    if self.rest_secs == 0 {
                        data.pause = true;

                        //play sound
                        data.command = Command::Sound(SoundCommand::Play);
                        data.command.process_with(&mut data.child_process);
                    }
                }
            }
        }
    }
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if data.reset_with_new_user_input == true {
            self.get_new_user_input(data);
            data.reset_with_new_user_input = false;
            self.skip_pause_next = true;
        }

        self.update_time(data);

        match data.session {
            Session::Work => {
                let degital_clock = format!(
                    "Work session: {:02}:{:02}:{:02}",
                    self.hours % 24,
                    self.mins % 60,
                    self.work_secs % 60
                );
                ui.label(
                    egui::RichText::new(degital_clock)
                        .color(egui::Color32::RED)
                        .code(),
                );
            }
            Session::Rest => {
                let degital_clock = format!(
                    "Rest session: {:02}:{:02}:{:02}",
                    self.hours % 24,
                    self.mins % 60,
                    self.rest_secs % 60
                );
                ui.label(
                    egui::RichText::new(degital_clock)
                        .color(egui::Color32::GREEN)
                        .code(),
                );
            }
        }
    }
}
