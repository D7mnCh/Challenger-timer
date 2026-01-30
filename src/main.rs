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

mod thread_counter;
use crate::thread_counter::*;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() -> eframe::Result {
    let (tx,rx) = mpsc::channel::<i32>();
    spawn_thread_counter(tx);
    // i wanna make that value get passed by reference to state struct
    // i think to solve my problem don't use channels
    let seconds = receive_secs(rx);

    /*
    loop{
        let seconds = rx.recv().unwrap();
        println!("this was printed from main: {}",seconds);
        thread::sleep(Duration::from_secs(1));
    }
    */

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
        Box::new(|cc| Ok(Box::new(State::new(cc,seconds)))),
    )
}

#[derive(Default)]
struct State {
    DeadComp: DeadComp,
    IntrComp: IntrComp,
    data: i32,
}

impl State {
    fn new(cc: &eframe::CreationContext<'_>,data: i32) -> Self {
        Default::default()
    }
    fn update_seconds() {
        //
    }
}

impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // udpate seconds 
        egui::CentralPanel::default().show(ctx, |ui| {
            /* .. */
            self.DeadComp.display(ui)
        });
    }
}
