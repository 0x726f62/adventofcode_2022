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

    fn disk_usage_with_threshold(&self, _: u32) -> u32 {
        self.size
    }

    fn find(&mut self, keyword: &str) -> Option<Box<&mut dyn FsItem>> {
        match self.name.eq(keyword) {
            true => Some(Box::new(self)),
            false => None,
        }
    }

    // fn find2(&mut self, _: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>> {
    //     if let Some(i) = item {
    //         match self.name.eq(i) {
    //             true => Some(Box::new(self)),
    //             false => None,
    //         }
    //     } else {
    //         None
    //     }
    // }

    fn find3(&mut self, _: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>> {
        //todo check also path if is <=1
        if let Some(i) = item {
            match self.name.eq(i) {
                true => Some(Box::new(self)),
                false => None,
            }
        } else {
            None
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

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_folder(&self) -> bool {
        false
    }
}