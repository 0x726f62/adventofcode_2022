use super::FsItem;

pub struct File<'a> {
    name: &'a str,
    size: u32,
    depth: u32,
}

impl<'a> File<'a>  {
    pub fn new(name: &'a str, size: u32, depth: u32) -> Self {
        Self { name, size, depth }
    }
}

impl<'a> FsItem for File<'a> {
    fn disk_usage(&self) -> u32{
        self.size
    }

    fn find(&self, keyword: &str) -> Option<&dyn FsItem> {
        match self.name.eq(keyword) {
            true => Some(self),
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

    // fn add(&mut self, fs_item: impl FsItem) {
    //     panic!();
    // }
}