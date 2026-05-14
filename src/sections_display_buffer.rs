pub struct SectionsDisplayBuffer {
    pub grid: Vec<Vec<DisplayElement>>,
}

impl SectionsDisplayBuffer {
    pub fn new() -> Self {
        Self { grid: vec![] }
    }

    pub fn set_size(&mut self) {
        let (width, height) = crossterm::terminal::size().unwrap();

        self.grid.clear();
        for _y in 0..height {
            let mut arr = vec![];
            for _x in 0..width {
                arr.push(DisplayElement::Space);
            }
            self.grid.push(arr);
        }
    }

    pub fn fill(&mut self, x: usize, y: usize, element: DisplayElement) {
        self.grid[y][x] = element;
    }
}

pub enum DisplayElement {
    Space,
    Border,
    Char(char),
}
