use crate::data::*;

#[derive(Default)]
pub struct TurnOffSoundButton;
impl TurnOffSoundButton {
    pub fn display(&mut self, ui: &mut egui::Ui, data: &mut Data) {
        if data.command == Command::Sound(SoundCommand::Play) {
            if ui.button("Turn off sound").clicked() {
                data.command = Command::Sound(SoundCommand::TurnOff);
                data.command.process_with(&mut data.child_process);
            }
        }
    }
}

