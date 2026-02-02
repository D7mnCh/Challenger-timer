mod static_components;
use crate::static_components::rest_cell::*;
use crate::static_components::switch_cell::*;
use crate::static_components::work_cell::*;
use crate::static_components::*;

mod intrective_components;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::rest_button::*;
use crate::intrective_components::work_button::*;
use crate::intrective_components::*;

use std::time::*;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            inner_size: Some(egui::Vec2 { x: 400.0, y: 400.0 }),
            title: Some("Timer".to_owned()),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "Timer",
        native_options,
        Box::new(|cc| Ok(Box::new(State::new(cc)))),
    )
}
#[derive(Debug)]
//struct Session; ??????
struct Data {
    /*
     // those two are seperate, the user can change work seconds and rest seconds
       rest_secs: u64,
       work_secs: u64
    */
    new_user_input: bool,
    pause : bool,
    instant: Instant,
    secs: u64,
    mins: u64,
    hours: u64,
}
struct State {
    static_comp: StaticComp,
    intr_comp: IntrComp,
    data: Data,
}
impl State {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            data: Data {
                instant: Instant::now(),
                new_user_input: true,
                pause: false,
                secs: 120,
                mins: 0,
                hours: 0,
            },
            static_comp: StaticComp {
                ..Default::default()
            },
            intr_comp: IntrComp {
                ..Default::default()
            },
        }
    }
    fn update_time_meseurment(&mut self) {
        if self.data.instant.elapsed().as_secs() == 1 {
            self.data.secs    += self.data.instant.elapsed().as_secs();
            self.data.mins    = self.data.secs / 60;
            self.data.hours   = self.data.secs / (60 * 60);
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        self.update_time_meseurment();

        egui::CentralPanel::default().show(ctx, |ui| {
            // whait, i can just the whole data struct, didn't think of that
            self.intr_comp.display(
                ui,
                &mut self.data
            );
            self.static_comp.display(
                ui,
                &mut self.data
            );
        });

        self.data.new_user_input = false;
        if self.data.instant.elapsed().as_secs() == 1 {
            self.data.instant = Instant::now();
        }
    }
}
/*
    - redisgne some of the main code
    - finish half switch_cell file dispaly time decrement 
    - add switch_buttons for working and rest session
    - make switch to work session reset the timer 
*/
