pub mod rest_cell;
pub mod switch_cell;
pub mod work_cell;

use crate::RestCell;
use crate::SwitchCell;
use crate::WorkCell;

#[derive(Default)]
pub struct DeadComp {
    work_cell: WorkCell,
    rest_cell: RestCell,
    switch_cell: SwitchCell,
}
impl DeadComp {
    pub fn display(&self, ui: &mut egui::Ui) {
        self.work_cell.display(ui);
        self.rest_cell.display(ui);
        self.switch_cell.display(ui);
    }
}
