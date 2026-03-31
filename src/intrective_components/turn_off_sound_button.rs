use crate::data::*;

#[derive(Default)]
pub struct TurnOffSoundButton;
// TODO if the user make seconds 0 and spam Play button, will spawn new sound
//on every press and can't turn them off with the turn off buttom
// maybe make the user not capable of make Rest/Work to 0 to fix it ? (i will
//do that)
// TODO if the sound finish, the buttom will still there (not disappeared)
// maybe check if chilled is killed, if yes then make the buttom disapeared ?
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
