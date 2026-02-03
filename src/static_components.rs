pub mod rest_cell;
pub mod switch_cell;
pub mod work_cell;

use crate::RestCell;
use crate::SwitchCell;
use crate::WorkCell;
// same crate ?, but the underneath one is from src/, what i know is crate is where the path start
use crate::Data;

#[derive(Default)]
pub struct StaticComp {
    //those two structs gave there own seconds
    pub work_cell: WorkCell,
    pub rest_cell: RestCell,
    pub switch_cell: SwitchCell,
}
impl StaticComp {
    pub fn display(&mut self, ui: &mut egui::Ui,data : &mut Data) {
        self.work_cell.display(ui, data);
        self.rest_cell.display(ui, data);
        self.switch_cell.display(ui, data);
    }
}
