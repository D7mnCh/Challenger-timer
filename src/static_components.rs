pub mod rest_cell;
pub mod switch_cell;
pub mod work_cell;

use crate::RestCell;
use crate::SwitchCell;
use crate::WorkCell;
use crate::Data;

#[derive(Default)]
pub struct StaticComp {
    pub work_cell: WorkCell,
    pub rest_cell: RestCell,
    pub switch_cell: SwitchCell,
}
impl StaticComp {
    pub fn display(&self, ui: &mut egui::Ui,data : &mut Data) {
        self.work_cell.display(ui, data);
        self.rest_cell.display(ui, data);
        self.switch_cell.display(ui, data);
    }
}
