use crate::{column::Column, scrollbar};

use sdl2::{
    pixels::Color,
    sys::{SDL_CreateSystemCursor, SDL_SetCursor},
};

pub struct Table {
    pub columns: Vec<Column>,
    pub selected_column: i32,
    pub selected_row: i32,
    pub hovered_column: i32,
    pub hovered_row: i32,
    pub default_cell_width: u32,
    pub default_row_height: u32,
    pub row_height: Vec<u32>,
    pub default_text_size: u16,
    pub on_column_edge: i32,
    pub x: i32,
    pub y: i32,
    pub horizontal_scrollbar: scrollbar::Scrollbar,
    pub vertical_scrollbar: scrollbar::Scrollbar,
}

impl Table {
    pub fn new() -> Table {
        let mut temp = Table {
            columns: Vec::new(),
            selected_column: -1,
            selected_row: -1,
            hovered_column: -1,
            hovered_row: -1,
            default_cell_width: 160,
            default_row_height: 30,
            row_height: [].to_vec(),
            default_text_size: 24,
            on_column_edge: -1,
            x: 0,
            y: 0,
            horizontal_scrollbar: scrollbar::Scrollbar::new(scrollbar::Orientation::Horizontal),
            vertical_scrollbar: scrollbar::Scrollbar::new(scrollbar::Orientation::Vertical),
        };
        temp.columns.push(Column::new(0, temp.default_row_height, 1, true));
        temp.row_height.push(temp.default_row_height);
        temp
    }

    pub fn add_column(&mut self) {
        let temp = self.columns.pop().unwrap();
        self.columns.push(Column::new(
            self.columns.len() as u32,
            self.default_cell_width,
            temp.cells.len() as u32,
            false,
        ));
        self.columns.push(temp);

        self.selected_column = -1;
        self.selected_row = -1;
        self.hovered_column = -1;
        self.hovered_row = -1;
            }

    pub fn resize(&mut self, mouse_delta: i32) {
        if self.on_column_edge != -1 {
            self.columns[self.on_column_edge as usize].resize(mouse_delta);
        }
    }

    pub fn test_scrollbar_click_down(
        &mut self,
        m_x: i32,
        m_y: i32,
        window_width: u32,
        window_height: u32,
        delta_x: i32,
        delta_y: i32,
    ) {
        self.horizontal_scrollbar
            .test_click(m_x, m_y, window_width, window_height, self.get_width(), self.get_height(), delta_x, delta_y);

        self.vertical_scrollbar
            .test_click(m_x, m_y, window_width, window_height, self.get_width(), self.get_height(), delta_x, delta_y);
    }

    pub fn select(&mut self, m_x: i32, m_y: i32) {
        let mut current_item_x = 0;

        for column in 0..self.columns.iter().len() - 1 {
            if m_x >= current_item_x && (m_x < current_item_x + self.columns[column].width as i32) {
                let mut current_item_y = self.default_row_height as i32;

                for row in 0..self.columns[0].cells.len() - 1 {
                    if m_y >= current_item_y && (m_y < current_item_y + self.row_height[row] as i32)
                    {
                        self.selected_column = column as i32;
                        self.selected_row = row as i32;
                        return;
                    }
                    current_item_y += self.row_height[row] as i32;
                }
            }
            current_item_x += self.columns[column].width as i32;
        }
        self.selected_column = -1;
        self.selected_row = -1;
    }

    pub fn reset_scrolls(&mut self) {
        self.horizontal_scrollbar.value = 0.0;
        self.vertical_scrollbar.value = 0.0;
    }

    pub fn check_hover_on_edge(&mut self, m_x: i32, m_y: i32) {
        let mut current_item_x = 0;
        for column in 0..self.columns.iter().len() - 1 {
            current_item_x += self.columns[column].width as i32;
            if m_x > current_item_x - 10 && m_x < current_item_x + 10 {
                for row in 0..self.columns[0].cells.len() {
                    let mut current_item_y = 0;
                    for _ in 0..self.columns.iter().len() {
                        current_item_y += self.row_height[row] as i32;
                    }
                    if m_y < current_item_y {
                        self.on_column_edge = column as i32;
                        unsafe {
                            SDL_SetCursor(SDL_CreateSystemCursor(
                                sdl2::sys::SDL_SystemCursor::SDL_SYSTEM_CURSOR_SIZEWE,
                            ));
                        }
                        return;
                    }
                }
            }
        }
        self.on_column_edge = -1;
        unsafe {
            SDL_SetCursor(SDL_CreateSystemCursor(
                sdl2::sys::SDL_SystemCursor::SDL_SYSTEM_CURSOR_ARROW,
            ));
        }
    }

    pub fn check_hover(&mut self, m_x: i32, m_y: i32) {
        let mut current_item_x = 0;
        for column in 0..self.columns.iter().len() {
            if m_x >= current_item_x && m_x < current_item_x + self.columns[column].width as i32 {
                let mut current_item_y = self.default_row_height as i32;
                for row in 0..self.columns[0].cells.len() {
                    if m_y >= current_item_y && m_y < current_item_y + self.row_height[row] as i32
                    {
                        self.hovered_row = row as i32;
                        self.hovered_column = column as i32;
                        return;
                    }
                    current_item_y += self.row_height[row] as i32;
                }
            }
            current_item_x += self.columns[column].width as i32;
        }
        self.hovered_column = -1;
        self.hovered_row = -1;
    }

    pub fn get_width(&self) -> u32 {
        let mut width = 0;
        for column in 0..self.columns.iter().len() {
            width += self.columns[column].width as u32;
        }
        width
    }

    pub fn get_height(&self) -> u32 {
        let mut height = self.default_row_height as u32;
        for row in 0..self.columns[0].cells.len() {
            height += self.row_height[row] as u32;
        }
        height
    }

    pub fn typing(&mut self, text: String) {
        self.columns[self.selected_column as usize]
            .get_selected(self.selected_row)
            .write_text(text); //self.columns[self.selected_column as usize].get_selected(self.selected_row).text.push(key); ;
    }

    pub fn has_selected(&self) -> bool {
        self.selected_column != -1 && self.selected_row != -1
    }

    pub fn has_hover(&self) -> bool {
        self.hovered_column != -1 && self.hovered_row != -1
    }

    pub fn display(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, window_width: u32, window_height: u32) {
        let mut x = -(self.horizontal_scrollbar.value * (self.get_width() as f32 / window_width as f32)) as i32;
        let y = -(self.vertical_scrollbar.value * (self.get_height() as f32 / window_height as f32)) as i32;
        for column in 0..self.columns.iter().len() {
            self.columns[column].display(
                canvas,
                x,
                y,
                self.selected_column == column as i32,
                self.selected_row,
                self.hovered_column == column as i32,
                self.hovered_row,
                &self.row_height,
                self.default_row_height,
                self.default_text_size,
                column == self.columns.len() - 1,
            );
            x += self.columns[column].width;
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .fill_rect(sdl2::rect::Rect::new(self.get_width() as i32, 0, 800, 600))
            .unwrap();

        self.horizontal_scrollbar.display(canvas, self.get_width(), window_width, self.get_height(), window_height);
        self.vertical_scrollbar.display(canvas, self.get_width(), window_width, self.get_height(), window_height);
    }

    pub fn add_row(&mut self) {
        for column in self.columns.iter_mut() {
            let temp = column.cells.pop().unwrap();
            column.add_cell();
            column.cells.push(temp)
        }
        let temp = self.row_height.pop().unwrap();
        self.row_height.push(self.default_row_height);
        self.row_height.push(temp);

        self.selected_column = -1;
        self.selected_row = -1;
        self.hovered_column = -1;
        self.hovered_row = -1;
    }

    pub fn test_add(&mut self, m_x: i32, m_y: i32) {
        let mut table_height = 0;
        for row in 0..self.columns[0].cells.len() {
            table_height += self.row_height[row] as i32;
        }
        let mut table_width = 0;
        for column in 0..self.columns.iter().len() - 1 {
            table_width += self.columns[column].width as i32;
        }
        if m_y > table_height && m_y < table_height + self.default_row_height as i32 && m_x < table_width + 30 {
            self.add_row();
        }
        if m_x > table_width && m_x < table_width + 30 as i32 &&  m_y < table_height + self.default_row_height as i32 {
            self.add_column();
        }

    }
}
