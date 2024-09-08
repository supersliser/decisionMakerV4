use crate::cell;

pub struct Cell_Controller {
    pub cell: *mut cell::Cell,
}

impl Cell_Controller {
    pub fn new(cell: *mut cell::Cell) -> Cell_Controller {
        Cell_Controller { cell }
    }
}