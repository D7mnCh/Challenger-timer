mod dead_components;
use crate::dead_components::rest_cell::*;
use crate::dead_components::switch_cell::*;
use crate::dead_components::work_cell::*;
use crate::dead_components::*;

mod intrective_components;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::rest_button::*;
use crate::intrective_components::work_button::*;
use crate::intrective_components::*;

fn main() -> eframe::Result {
    // native options will modify the native window's behavior
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

#[derive(Default)]
struct State {
    DeadComp: DeadComp,
    IntrComp: IntrComp,
}

impl State {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            /* .. */
            self.DeadComp.display(ui)
        });
    }
}
