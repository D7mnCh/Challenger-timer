use crate::data::*;

// NOTE i think switch cell is now intrective and not static, when i drag the value
//it will update
#[derive(Default, Debug)]
pub struct SwitchCell {
    work_secs: u64,
    rest_secs: u64,
    mins: u64,
    hours: u64,
    round: Round,
}
#[derive(Default, Debug, PartialEq)]
pub enum Round {
    #[default]
    Main,
    Bonus,
    //NOTE if you add it you can move self.round match arm into the end of data.session
    //match arm
    //ShortRest
    //LongRest
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
                    match self.round {
                        Round::Main => {
                            if self.work_secs == 0 {
                                //play sound
                                data.command = Command::PlaySound;
                                data.sound = Sound::MainRoundFinished;
                                data.command
                                    .process_with(&mut data.child_process, &mut data.sound);

                                self.round = Round::Bonus;
                                self.work_secs = 900;
                            }
                        }

                        Round::Bonus => {
                            if self.work_secs == 0 {
                                //play sound
                                data.command = Command::PlaySound;
                                data.sound = Sound::BonusRoundFinished;
                                data.command
                                    .process_with(&mut data.child_process, &mut data.sound);

                                data.pause = true;
                                self.round = Round::Main;
                                data.reset_with_new_user_input = true;
                            }
                        }
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
                        data.sound = Sound::Rest;
                        data.command = Command::PlaySound;
                        data.command
                            .process_with(&mut data.child_process, &mut data.sound);
                    }
                }
            }
        }
    }
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if data.reset_with_new_user_input == true {
            self.get_new_user_input(data);
            data.reset_with_new_user_input = false;
        }

        self.update_time(data);

        let (color, which_session, secs) = match data.session {
            Session::Work => (data.work_color, "Work", self.work_secs),
            Session::Rest => (data.rest_color, "Rest", self.rest_secs),
        };
        let degital_clock = format!(
            "{} session: {:02}:{:02}:{:02}",
            which_session,
            self.hours % 24,
            self.mins % 60,
            secs % 60,
        );
        let mut cell_size;
        if data.pause == false {
            cell_size = 60.0;
        } else {
            cell_size = 16.0;
        }

        ui.label(
            egui::RichText::new(degital_clock)
                .color(color)
                .strong()
                .size(cell_size),
        );
    }
}
