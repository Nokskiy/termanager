use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};

pub struct DisplayBuffer {
    grid: Vec<Vec<DisplayBufferElement>>,
}

impl DisplayBuffer {
    pub fn new() -> Self {
        Self { grid: vec![] }
    }

    pub fn update_grid(&mut self) {
        let (terminal_width, terminal_height) = terminal::size().unwrap();

        let mut width_vec = vec![];
        width_vec.resize_with(terminal_width as usize, || DisplayBufferElement::None);

        self.grid
            .resize_with(terminal_height as usize, || width_vec.clone());
    }

    pub fn set_cell(&mut self, x: usize, y: usize, display_element: DisplayBufferElement) {
        self.grid[y][x] = display_element;
    }

    pub fn draw(self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let element = &self.grid[y][x];
                match element {
                    DisplayBufferElement::None => {}

                    DisplayBufferElement::Char(char) => execute!(
                        stdout(),
                        MoveTo(x as u16, y as u16),
                        SetBackgroundColor(Color::Red),
                        SetForegroundColor(Color::Red),
                        Print(char)
                    )
                    .unwrap(),

                    DisplayBufferElement::Border => execute!(
                        stdout(),
                        MoveTo(x as u16, y as u16),
                        SetBackgroundColor(Color::Blue),
                        SetForegroundColor(Color::Blue),
                        Print(" ")
                    )
                    .unwrap(),
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum DisplayBufferElement {
    None,
    Char(char),
    Border,
}
