use super::FsItem;

pub struct File {
    name: &'static str,
    size: u32,
}

impl File {
    pub fn new(name: &'static str, size: u32) -> Self {
        Self { name, size }
    }
}

impl FsItem for File {
    fn disk_usage(&self) -> u32{
        self.size
    }

    fn find(&self, keyword: &str) -> bool {
        self.name.eq(keyword)
    }
}