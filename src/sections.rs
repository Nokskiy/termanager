pub trait Section {
    fn display_section(&self);
}

pub struct FilesSection {}

impl FilesSection {
    pub fn new() -> Box<dyn Section + Send> {
        Box::new(Self {})
    }
}

impl Section for FilesSection {
    fn display_section(&self) {
        println!("Section")
    }
}
