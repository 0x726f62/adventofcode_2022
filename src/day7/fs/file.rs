use super::FsItem;

pub struct File<'a> {
    name: &'a str,
    size: u32,
}

impl<'a> File<'a>  {
    pub fn new(name: &'a str, size: u32) -> Self {
        Self { name, size }
    }
}

impl<'a> FsItem for File<'a> {
    fn disk_usage(&self) -> u32{
        self.size
    }

    fn find(&self, keyword: &str) -> bool {
        self.name.eq(keyword)
    }
}