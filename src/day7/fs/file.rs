use super::FsItem;

pub struct File {
    name: String,
    size: u32,
    depth: u32,
}

impl File {
    pub fn new(name: String, size: u32, depth: u32) -> Self {
        Self { name, size, depth }
    }
}

impl FsItem for File {
    fn disk_usage(&self) -> u32{
        self.size
    }

    fn find(&mut self, keyword: &str) -> Option<Box<&mut dyn FsItem>> {
        match self.name.eq(keyword) {
            true => Some(Box::new(self)),
            false => None,
        }
    }

    fn print(&self) {
        for _ in 0..self.depth {
            print!("-");
        }
        println!(" FILE {}", self.name);
    }

    fn depth(&self) -> u32 {
        self.depth
    }

    fn add(&mut self, _fs_item: Box<dyn FsItem>) {
        panic!();
    }
}