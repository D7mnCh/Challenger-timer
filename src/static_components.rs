pub mod rest_cell;
pub mod switch_cell;
pub mod work_cell;

use crate::RestCell;
use crate::SwitchCell;
use crate::WorkCell;

#[derive(Default)]
pub struct StaticComp {
   pub work_cell: WorkCell,
   pub rest_cell: RestCell,
   pub switch_cell: SwitchCell,
}
impl StaticComp {
    pub fn display(&self, ui: &mut egui::Ui) {
        self.work_cell.display(ui);
        self.rest_cell.display(ui);
        self.switch_cell.display(ui);
    }
}
