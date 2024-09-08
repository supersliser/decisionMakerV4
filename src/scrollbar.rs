#[derive(PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Scrollbar {
    pub value: f32,
    pub orientation: Orientation,
}

impl Scrollbar {
    pub fn new(orientation: Orientation) -> Self {
        Scrollbar {
            value: 0.0,
            orientation: orientation,
        }
    }

    pub fn test_click(
        &mut self,
        m_x: i32,
        m_y: i32,
        window_width: u32,
        window_height: u32,
        table_width: u32,
        table_height: u32,
        delta_x: i32,
        delta_y: i32,
    ) {
        if self.orientation == Orientation::Horizontal {
            if table_width < window_width as u32 {
                return;
            }
            if m_x >= 0
                && m_y >= window_height as i32 - 20
                && m_x <= window_width as i32
                && m_y <= window_height as i32
            {
                self.move_bar(
                    delta_x,
                    delta_y,
                    table_width,
                    window_width,
                    table_height,
                    window_height,
                );
            }
        } else {
            if table_height < window_height as u32 {
                return;
            }
            if m_y >= 0
                && m_x >= window_width as i32 - 20
                && m_y <= window_height as i32
                && m_x <= window_width as i32
            {
                self.move_bar(
                    delta_x,
                    delta_y,
                    table_width,
                    window_width,
                    table_height,
                    window_height,
                );
            }
        }
    }

    pub fn move_bar(
        &mut self,
        delta_x: i32,
        delta_y: i32,
        table_width: u32,
        window_width: u32,
        table_height: u32,
        window_height: u32,
    ) {
        if self.orientation == Orientation::Horizontal {
            self.value += delta_x as f32;
            if self.value < 0.0 {
                self.value = 0.0;
            }
            if self.value
                + (window_width as f32 / table_width as f32) * window_width as f32
                >= window_width as f32
            {
                self.value = window_width as f32 - (window_width as f32 / table_width as f32) * window_width as f32;
            }
        }
        if self.orientation == Orientation::Vertical {
            self.value += delta_y as f32;
            if self.value < 0.0 {
                self.value = 0.0;
            }
            if self.value
                + (window_height as f32 / table_height as f32) * window_height as f32
                >= window_height as f32
            {
                self.value = window_height as f32 - (window_height as f32 / table_height as f32) * window_height as f32;
            }
        }
    }

    pub fn display(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        table_width: u32,
        window_width: u32,
        table_height: u32,
        window_height: u32,
    ) {
        if self.orientation == Orientation::Horizontal {
            if window_width < table_width {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        0,
                        window_height as i32 - 20,
                        window_width,
                        20,
                    ))
                    .unwrap();

                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas
                    .draw_rect(sdl2::rect::Rect::new(
                        0,
                        window_height as i32 - 20,
                        window_width,
                        20,
                    ))
                    .unwrap();
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        self.value as i32,
                        window_height as i32 - 20,
                        ((window_width as f32 / table_width as f32) * window_width as f32) as u32,
                        20,
                    ))
                    .unwrap();
            }
        }

        if self.orientation == Orientation::Vertical {
            if window_height < table_height {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        window_width as i32 - 20,
                        0,
                        20,
                        window_height,
                    ))
                    .unwrap();
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas
                    .draw_rect(sdl2::rect::Rect::new(
                        window_width as i32 - 20,
                        0,
                        20,
                        window_height,
                    ))
                    .unwrap();
                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                canvas
                    .fill_rect(sdl2::rect::Rect::new(
                        window_width as i32 - 20,
                        self.value as i32,
                        20,
                        ((window_height as f32 / table_height as f32) * window_height as f32) as u32,
                    ))
                    .unwrap();
            }
        }
    }
}
