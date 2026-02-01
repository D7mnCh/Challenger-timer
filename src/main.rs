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
struct Data {
    reference_instant: Instant,
    seconds: u64,
    minutes: u64,
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
            data: Data{
                reference_instant: Instant::now(),
                seconds: 0,
                minutes: 0,
                hours: 0,
            },
            static_comp: StaticComp {
                ..Default::default()
            },
            intr_comp: IntrComp{
                ..Default::default()
            }
        }
    }
    fn update_time_meseurment(&mut self) {
        // Math logic
        self.data.seconds = self.data.reference_instant.elapsed().as_secs() /* + self.data.secs */;
        self.data.minutes = self.data.seconds / 60 ; 
        self.data.hours   = self.data.seconds / (60 * 60) ;
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.update_time_meseurment();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.static_comp.display(
                ui,
                &mut self.data.seconds,
                &self.data.minutes, 
                &self.data.hours);
            self.intr_comp.display(
                ui,
                &mut self.data.seconds,
                &self.data.minutes, 
                &self.data.hours)
        });
    }
}
/* 
    the plan
 - put the data in the structs that use it
 - ok so i faced a problem of the update don't appear only if i haver my mouse the window, or when i made a keyboard input (solved)
 - implementing buttoms to stop the secs (i could use other Instant checkpoint thing and then substract it on the main one)
*/

