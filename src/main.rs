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
#[derive(Clone)]
struct Data {
    // i think i need pause to be here in data that all components can share and interect with it
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
                secs: 0,
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

            self.data.instant = Instant::now();
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_time_meseurment();
        //
        // begin here and solve this
        // i wanna make self.data.instant have 1 as a value not bigger then that
        // the self.data.secs have normal value or in other word he's the only who own the value of second
        // i get an idea to updata it everytime here, using plus one to self.data.instant cause it will be always be zero
        egui::CentralPanel::default().show(ctx, |ui| {
            // whait, i can just the whole data struct, didn't think of that
            self.static_comp.display(
                ui,
                &mut self.data
            );
            self.intr_comp.display(
                ui,
                &mut self.data
            )
        });
    }
}
/*
 *
    the plan
 - Update a bit README
 - passing arguments using data instance of Data struct, and not via passing all it fields -.-
 - implemented button logic to stop the timer
 - trying to implement countdown on SwitchCell that the timer will get denceared
 - i changed a bit how seconds update, just one field that holds it, not seperated through 2 fields

*/
