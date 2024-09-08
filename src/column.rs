use crate::cell::Cell;

pub struct Column {
    pub name: String,
    pub width: i32,
    pub cells: Vec<Cell>,
}

impl Column {
    pub fn new(index: u32, default_width: u32, rows: u32, on_last: bool) -> Column {
        let mut temp: Column;
        if on_last {
            temp = Column {
                name: "".to_string(),
                width: default_width as i32,
                cells: Vec::new(),
            };
        } else {
            temp = Column {
                name: format!("Column {}", index),
                width: default_width as i32,
                cells: Vec::new(),
            };
        }


        for _ in 0..rows {
            temp.add_cell();
        }

        temp
    }

    pub fn set_name(&mut self, text: String) {
        self.name = text;
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_cell(&mut self) {
        self.cells.push(Cell::new());
    }

    pub fn is_mouse_on_edge(&self, m_x: i32, m_y: i32) -> bool {
        m_x < 0 || m_y < 0 || m_x > self.width as i32 || m_y > self.cells.len() as i32 * 40
    }

    pub fn resize(&mut self, mouse_delta: i32) {
        if self.width > 20 || mouse_delta > 0 {
            self.width += mouse_delta;
        }
    }

    pub fn get_selected(&mut self, row: i32) -> &mut Cell {
        self.cells.get_mut(row as usize).unwrap()
    }

    pub fn display(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        x: i32,
        y: i32,
        col_selected: bool,
        row_selected: i32,
        col_hovered: bool,
        row_hovered: i32,
        cell_height: *const Vec<u32>,
        default_cell_height: u32,
        text_size: u16,
        on_last: bool,
    ) {
        let mut head_cell = Cell::new();
        head_cell.set_text(self.get_name());
        let _ = head_cell.display(
            canvas,
            x,
            y,
            self.width as u32,
            default_cell_height,
            false,
            col_selected,
            col_hovered,
            text_size,
            on_last,
        );
        let mut y = default_cell_height as i32 + y;
        unsafe {
            for (i, cell) in self.cells.iter().enumerate() {
                let _ = cell.display(
                    canvas,
                    x,
                    y,
                    self.width as u32,
                    (*cell_height)[i],
                    col_selected && i as i32 == row_selected,
                    row_selected == i as i32 || col_selected,
                    row_hovered == i as i32 || col_hovered,
                    text_size,
                    i == self.cells.len() - 1 || on_last,
                );
                y += (*cell_height)[i] as i32
            }
        }
    }
}
