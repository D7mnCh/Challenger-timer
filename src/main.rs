mod data;
use crate::data::*;

mod static_components;
use crate::static_components::rest_cell::*;
use crate::static_components::switch_cell::*;
use crate::static_components::work_cell::*;
use crate::static_components::*;

mod intrective_components;
use crate::intrective_components::IntrComp;
use crate::intrective_components::pause_button::*;
use crate::intrective_components::rest_button::*;
use crate::intrective_components::rest_secs_glider::*;
use crate::intrective_components::work_button::*;
use crate::intrective_components::work_secs_glider::*;

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
                //reset: false,
                pause: true,
                session: Session::Work,
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
            /*
            self.intr_comp.display(
            ui,
            &mut self.data
            );
            self.static_comp.display(
            ui,
            &mut self.data
            );
            */
            // i think the bugs is hepping is because the functions mutatable borrow this ui thing
            // i think it's only space, read what the author said about it
            //ui.take_available_width();
            ui.vertical(|ui| {

                ui.add_space(ctx.viewport_rect().max.y / 3.);

                ui.horizontal(|ui| {

                    ui.add_space(ctx.viewport_rect().max.x / 3.7);

                    self.static_comp.switch_cell.display(ui, &mut self.data);
                });

                ui.add_space(10.);

                ui.horizontal(|ui| {

                    self.intr_comp.rest_button.display(ui, &mut self.data);

                    //ui.add_space(10.);

                    self.intr_comp.pause_button.display(ui, &mut self.data);

                    //ui.add_space(10.);

                    self.intr_comp.work_button.display(ui, &mut self.data);

                    ui.add_space(ctx.viewport_rect().max.x / 50.);
                });

                ui.add_space(10.);

                ui.horizontal(|ui| {
                    ui.add_space(ctx.viewport_rect().max.x / 3.5);

                    self.intr_comp.rest_secs_glider.display(ui, &mut self.data);
                    self.intr_comp.work_secs_glider.display(ui, &mut self.data);
                });

                ui.add_space(ctx.viewport_rect().max.y / 3.);

                ui.horizontal(|ui| {
                    self.static_comp.work_cell.display(ui, &mut self.data);

                    ui.add_space(ctx.viewport_rect().max.x / 2.8);

                    self.static_comp.rest_cell.display(ui, &mut self.data);
                });
            });
        });

        if self.data.instant.elapsed().as_secs() == 1 {
            self.data.instant = std::time::Instant::now();
        }
    }
}
/*
 - Redesign the code base so i can controll where i can put the ui (positioning)
 TODO
 - don't know how to finish this project, i am kinda want the pommodoro
 - if you want to fix Ui, for now fix the look not the possion, if you want to fully fix it, you need to redesign your code
    - what i know for location you need to depend on Layouts, and for size use "add_sized" instead of "add"
 - maybe add audio ?
*/
