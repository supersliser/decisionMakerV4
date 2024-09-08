use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::sys::ttf::TTF_SizeText;

pub struct Cell {
    pub text: String,
    pub cursor: i32,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            text: "".to_string(),
            cursor: 0,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.cursor = self.text.len() as i32 - 1;
    }

    pub fn write_text(&mut self, text: String) {

        let temp = self.text[0..self.cursor as usize].to_string();
        self.text = temp + &text + &self.text[self.cursor as usize..self.text.len()];
        self.cursor += 1
    }
    pub fn delete_text(&mut self) {
        if self.cursor > 0 {
            self.text.remove(self.cursor as usize - 1);
            self.cursor -= 1;
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn shift_cursor(&mut self, delta: i32) {
        if self.cursor + delta < 0 {
            self.cursor = 0;
        }

        if self.cursor + delta > self.text.len() as i32 - 1 {
            self.cursor = self.text.len() as i32 - 1;
        }
        self.cursor += delta;
    }

    pub fn display(
        &self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        selected: bool,
        partial_selected: bool,
        hovered: bool,
        text_size: u16,
        on_last: bool,
    ) -> Result<(), String> {
        let color = if on_last {
            sdl2::pixels::Color::RGB(0, 128, 0)
        } else if selected {
            sdl2::pixels::Color::RGB(14, 123, 196)
        } else if partial_selected {
            sdl2::pixels::Color::RGB(42, 164, 245)
        } else if hovered {
            sdl2::pixels::Color::RGB(132, 196, 240)
        } else {
            sdl2::pixels::Color::RGB(20, 20, 20)
        };

        canvas.set_draw_color(color);
        canvas
            .fill_rect(sdl2::rect::Rect::new(x as i32, y as i32, width, height))
            .unwrap();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas
            .draw_rect(sdl2::rect::Rect::new(x as i32, y as i32, width, height))
            .unwrap();

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        // Load a font
        let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
        let mut font = ttf_context.load_font(font_path, text_size)?;
        font.set_style(sdl2::ttf::FontStyle::BOLD);

        let surface = font
            .render(&self.text)
            .blended_wrapped(Color::RGBA(255, 255, 255, 255), 10000)
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();

        canvas
            .copy(&texture, None, Some(Rect::new(x as i32, y as i32, width, height)))
            .unwrap();

        if selected {
            let mut size_x = 0;
            let mut size_y = 0;
            for i in 0..self.cursor as i32 {
                if self.text.chars().nth(i as usize).unwrap() == '\n' {
                    size_x = 0;
                    size_y += font.size_of_char(self.text.chars().nth(i as usize).unwrap()).unwrap().1 as i32;
                } else {
                    size_x += font.size_of_char(self.text.chars().nth(i as usize).unwrap()).unwrap().0 as i32;
                }
            }
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                    canvas
                        .fill_rect(sdl2::rect::Rect::new(x as i32 + size_x, y as i32 + size_y, 2, height))
                        .unwrap();
        }
        


        Ok(())
    }
}
