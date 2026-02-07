#[derive(Debug)]
pub enum Session {
    Work,
    Rest,
}

#[derive(Debug,PartialEq)]
pub enum Command {
    Sound(SoundCommand),
    None,
}
#[derive(Debug,PartialEq)]
pub enum SoundCommand {
    Play, // i want this to give an id 
    TurnOff // kill sound with that id from Play  variant
}

#[derive(Debug)]
pub struct Data {
    pub reset_with_new_user_input: bool,
    pub pause: bool,
    pub instant: std::time::Instant,
    pub child_process: Option<std::process::Child>,

    pub session: Session,
    pub command: Command,

    pub rest_secs: u64,
    pub work_secs: u64,
}

impl Command {
    // acutally there is built in to kill that process, do i just over engeneering or something ?
    // https://doc.rust-lang.org/std/process/struct.Child.html#method.kill
    fn shutdown_sound(child_process: &mut Option<std::process::Child> ) {
        if let Some(child_process) = child_process {
            child_process.kill();
        }else if let None = child_process{
            println!("can't shutdown sound!")
        };
    }

    fn play_sound(child_process: &mut Option<std::process::Child>) {
        let path = std::env::home_dir()
            .expect("could not found home directory")
            .join("App");

        #[cfg(not(target_family = "windows"))]
        let command = std::process::Command::new("mpv")
            .current_dir(path)
            .args(["finish.wav","--no-audio-display"])
            .spawn()
            .expect("mpv failed to lunch");
        *child_process = Some(command);

        //i might have a problem here on windows platform
        #[cfg(target_family = "windows")]
        {
            let command = std::process::Command::new("powershell")
                .current_dir(path)
                .args(["-c","(New-Object Media.SoundPlayer '.\\finish.wav').PlaySync();" ])
                .spawn()
                .expect("powershell failed to start or the problem could be in args");

            *child_process = Some(command);
        }
    }

    pub fn process_with (&self, child_process: &mut Option<std::process::Child>) {
        match self {
            Command::Sound(sound_command) => match sound_command {
                SoundCommand::Play =>{ 
                    println!("sound on");
                    Self::play_sound(child_process);
                },
                // actually, this variant depend on the other one, i can't run it if the the sound
                // is off
                SoundCommand::TurnOff =>{
                    println!("sound turn off");
                    Self::shutdown_sound(child_process);
                }
            }
            Command::None => println!("No command at the moment"),
        }
    }
}
