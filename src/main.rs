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
enum Session {
    Work,
    Rest
}
#[derive(Debug)]
struct Data {
    // for this field gonna reset secs
    new_user_input: bool,
    //reset: bool,
    pause : bool,
    instant: Instant,
    session: Session,

    rest_secs: u64,
    work_secs: u64
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
                //reset: false,
                pause: false,
                session: Session::Work,
                rest_secs: 5,
                work_secs: 320,
            },
            static_comp: StaticComp {
                ..Default::default()
            },
            intr_comp: IntrComp {
                ..Default::default()
            },
        }
    }
    fn get_user_input (&mut self) {
        //..
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
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
 - made every cell have independing self secs
 - make switch work for rest and work session
 - finished work and rest cells (for rest session it act differently but i like it)
 - now user can manipulate secs both for rest and work

 TODO
 - maybe add audio ?
*/
