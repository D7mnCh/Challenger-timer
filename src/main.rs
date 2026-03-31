#![allow(warnings)]
mod data;
use crate::data::*;

mod static_components;
use crate::static_components::rest_cell::*;
use crate::static_components::switch_cell::*;
use crate::static_components::work_cell::*;
use crate::static_components::*;

mod intrective_components;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::rest_button::*;
use crate::intrective_components::rest_secs_glider::*;
use crate::intrective_components::turn_off_sound_button::*;
use crate::intrective_components::work_button::*;
use crate::intrective_components::work_secs_glider::*;
use crate::intrective_components::IntrComp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        //[1148.4063 163.8125] wide window
        //[370, 115] tiny floating window
        viewport: egui::ViewportBuilder {
            inner_size: Some(egui::Vec2 { x: 370.0, y: 115.0 }),
            title: Some("Timer".to_owned()),
            resizable: Some(false),
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
struct State {
    static_comp: StaticComp,
    intr_comp: IntrComp,
    data: Data,
}
impl State {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            data: Data {
                instant: std::time::Instant::now(),
                reset_with_new_user_input: true,
                pause: true,

                session: Session::Work,
                command: Command::None,
                child_process: None,

                rest_secs: 900,
                work_secs: 2700,
            },
            static_comp: StaticComp {
                ..Default::default()
            },
            intr_comp: IntrComp {
                ..Default::default()
            },
        }
    }
}
impl eframe::App for State {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.add_space(ctx.viewport_rect().max.x / 4.);

                    self.static_comp.switch_cell.display(ui, &mut self.data);
                });

                ui.add_space(10.);

                ui.horizontal(|ui| {
                    self.intr_comp.rest_button.display(ui, &mut self.data);
                    self.intr_comp.pause_button.display(ui, &mut self.data);
                    self.intr_comp.work_button.display(ui, &mut self.data);
                });

                // make it here
                ui.add_space(8.);

                ui.horizontal(|ui| {
                    ui.add_space(ctx.viewport_rect().max.x / 3.2);

                    self.intr_comp.rest_secs_glider.display(ui, &mut self.data);
                    self.intr_comp.work_secs_glider.display(ui, &mut self.data);
                    self.intr_comp
                        .turn_off_sound_button
                        .display(ui, &mut self.data);
                });

                ui.horizontal(|ui| {
                    self.static_comp.work_cell.display(ui, &mut self.data);

                    ui.add_space(ctx.viewport_rect().max.x / 3.2);

                    self.static_comp.rest_cell.display(ui, &mut self.data);
                });
            });
        });
        println!("{}", ctx.viewport_rect().max);
        // NOTE if device suspended, self.data.instant.elapsed().as_secs() will get
        // more then 1
        if self.data.instant.elapsed().as_secs() > 1 {
            self.data.pause = true;
            self.data.instant = std::time::Instant::now();
        }
        if self.data.instant.elapsed().as_secs() == 1 {
            self.data.instant = std::time::Instant::now();
        }
    }
}
/*
 TODO
 - maybe make the app on the buttom of the top
 - disable resizing
 - keyboard support
 - i think i can pass only the data that i need to some methods like, not passing all data methods into that method
 - web support
 - IO improvment
 NOTE (bugs)
 - if you exit the app with the midia player being played it will hid the cursor
    - `tput cnorm` to make cursor appear again
 - when suspending my pc, the app will crach with seconds went too fast to below 0(overflow) in less then 1 second
 - when the sound finished, make the buttom to disapear
*/
