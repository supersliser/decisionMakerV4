extern crate sdl2;

pub mod cell;
pub mod cell_controller;
pub mod column;
pub mod scrollbar;
pub mod table;

use sdl2::event::Event;
use sdl2::pixels::Color;

pub fn main() {
    let mut win_width = 800;
    let mut win_height = 600;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Decision Maker Version 4", win_width, win_height)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut table = table::Table::new();

    table.add_column();
    table.add_column();
    table.add_column();

    table.add_row();
    table.add_row();
    table.add_row();

    table.display(&mut canvas, win_width, win_height);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut mouse_held = false;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(width, height) => {
                        win_width = width as u32;
                        win_height = height as u32;
                        table.reset_scrolls();
                    }
                    _ => {}
                },
                Event::TextInput { text, .. } => {
                    if table.has_selected() {
                        table.typing(text);
                    }
                }
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    table.check_hover(x, y);
                    table.check_hover_on_edge(x, y);

                    if mouse_held {
                        table.resize(xrel);
                        table.test_scrollbar_click_down(x, y, win_width, win_height, xrel, yrel);
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    table.select(x, y);
                    table.test_add(x, y);
                    mouse_held = true;
                }
                Event::MouseButtonUp { .. } => {
                    mouse_held = false;
                }
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => {
                    if table.has_selected() {
                        match keycode.unwrap() {
                            sdl2::keyboard::Keycode::Backspace => {
                                table.columns[table.selected_column as usize]
                                    .get_selected(table.selected_row)
                                    .delete_text();
                            }
                            sdl2::keyboard::Keycode::Left => {
                                table.selected_column -= 1;
                            }
                            sdl2::keyboard::Keycode::Right => {
                                table.selected_column += 1;
                            }
                            sdl2::keyboard::Keycode::Up => {
                                table.selected_row -= 1;
                            }
                            sdl2::keyboard::Keycode::Down => {
                                table.selected_row += 1;
                            }
                            sdl2::keyboard::Keycode::Return => {
                                table.typing("\n".to_string());
                                table.row_height[table.selected_row as usize] += 30;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        table.display(&mut canvas, win_width, win_height);
        canvas.present();
    }
}
