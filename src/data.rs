#[derive(Debug)]
pub enum Session {
    Work,
    Rest,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Sound(SoundCommand),
    None,
}
#[derive(Debug, PartialEq)]
pub enum SoundCommand {
    Play,
    TurnOff,
}

#[derive(Debug)]
pub struct Data {
    pub reset_with_new_user_input: bool,
    pub pause: bool,
    pub reset_totals: bool,
    pub instant: std::time::Instant,

    pub session: Session,
    pub command: Command,
    pub child_process: Option<std::process::Child>,

    pub rest_secs: u64,
    pub work_secs: u64,
}

impl Command {
    fn shutdown_sound(child_process: &mut Option<std::process::Child>) {
        if let Some(child_process) = child_process.as_mut() {
            child_process.kill();
            // clean up the the zombie process after killed
            child_process.wait();
        } else if let None = child_process {
            println!("can't shutdown sound!")
        };
    }

    fn play_sound(child_process: &mut Option<std::process::Child>) {
        let path = std::env::current_dir().unwrap().join("sounds");
        //println!("The current directory is {}", path.display());

        //let path = std::env::home_dir()
        //    .expect("could not found home directory")
        //    .join("App");

        #[cfg(not(target_family = "windows"))]
        {
            let command = std::process::Command::new("mpv")
                .current_dir(path)
                .args(["finish.wav", "--no-audio-display"])
                .spawn()
                .expect("mpv failed to lunch");
            *child_process = Some(command);
        }

        #[cfg(target_family = "windows")]
        {
            println!("windows PATH : {path:?}");
            let command = std::process::Command::new("powershell")
                .current_dir(path)
                .args([
                    "-c",
                    "(New-Object Media.SoundPlayer '.\\finish.wav').PlaySync();",
                ])
                .spawn()
                .expect("powershell failed to start or the problem could be in args");

            *child_process = Some(command);
        }
    }

    pub fn process_with(&self, child_process: &mut Option<std::process::Child>) {
        match self {
            Command::Sound(sound_command) => match sound_command {
                SoundCommand::Play => {
                    println!("sound on");
                    Self::play_sound(child_process);
                }
                // actually, this variant depend on the other one, i can't run it if the the sound
                // is off
                SoundCommand::TurnOff => {
                    println!("\nsound turn off");
                    Self::shutdown_sound(child_process);
                }
            },
            Command::None => println!("No command at the moment"),
        }
    }
}
