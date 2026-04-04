#![allow(warnings)]
mod data;
use crate::data::{Command, Data, Session};

mod static_components;
use crate::static_components::rest_cell::*;
use crate::static_components::switch_cell::*;
use crate::static_components::work_cell::*;
use crate::static_components::*;

mod intrective_components;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::reset_totals::*;
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
            inner_size: Some(egui::Vec2 { x: 370.0, y: 195.0 }),
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
                reset_totals: false,

                session: Session::Work,
                command: Command::None,
                child_process: None,

                rest_secs: 300,
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
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    ui.add_space(90.0);
                    self.static_comp.switch_cell.display(ui, &mut self.data);
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.add_space(35.0);
                    ui.group(|ui| {
                        ui.add_space(75.0);
                        ui.vertical(|ui| {
                            self.static_comp.work_cell.display(ui, &mut self.data);
                            ui.add_space(5.0);
                            self.static_comp.rest_cell.display(ui, &mut self.data);
                        });
                        ui.add_space(75.0);
                    });
                });
                // NOTE didn't like i put this logic here where should be only
                //ui code
                if self.data.reset_totals == true {
                    self.data.reset_totals = false
                }

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    self.intr_comp.rest_button.display(ui, &mut self.data);
                    self.intr_comp.pause_button.display(ui, &mut self.data);
                    self.intr_comp.work_button.display(ui, &mut self.data);
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.add_space(115.0);
                    self.intr_comp.rest_secs_glider.display(ui, &mut self.data);
                    self.intr_comp.work_secs_glider.display(ui, &mut self.data);
                });

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    self.intr_comp.reset_totals.display(ui, &mut self.data);
                    ui.add_space(175.0);
                    self.intr_comp
                        .turn_off_sound_button
                        .display(ui, &mut self.data);
                });
            });
        });
        //println!("{}", ctx.viewport_rect().max);
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
 - redo the ui so i can make it at the top (full window at top) with an app
 running below it like btop
 - make other sounds than the current one
 - should make an enum for sounds
 - maybe if i finish the 45 mintes go to 15 and then when finsiehd go back to 45
 - maybe make the app on the buttom of the top
 - keyboard support
 - i think i can pass only the data that i need to some methods like, not passing all data methods into that method
 - web support
 - IO improvment
 - add click sounds
 - switch to 15 minutes after 45 minutes is done (don't make 15 minutes on
  rest session)
 - when the sound finished, make the buttom to disapear
 NOTE (bugs)
 - sometimes the timer will freeze for one second or even more then that
 - the program you are building is for you after all, the features you will add
 is based on your needs
 - if you exit the app with the midia player being played it will hid the cursor
    - `tput cnorm` to make cursor appear again
*/
