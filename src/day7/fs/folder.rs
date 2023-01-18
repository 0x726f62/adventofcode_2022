use super::FsItem;

pub struct Folder {
    name: &'static str,
    fs_items: Vec<Box<dyn FsItem>>,
}

impl Folder {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fs_items: vec![],
        }
    }

    pub fn add(&mut self, fs_item: impl FsItem + 'static) {
        self.fs_items.push(Box::new(fs_item));
    }
}

impl FsItem for Folder {
    fn disk_usage(&self) -> u32 {
        let mut sum: u32 = 0;
        for fs_item in self.fs_items.iter() {
            sum += fs_item.disk_usage();
        }

        sum
    }

    fn find(&self, keyword: &str) -> bool {
        //take the first
        let splits: Vec<&str> = keyword.split('/').collect();
        // compare with name
        if !self.name.eq(splits[0]) {
            // if not eq -> return false
            return false;
        } else if splits.len() == 1 {
            //case to look for folder existence
            return true;
        }

        for fs_item in self.fs_items.iter() {
            let skipped: Vec<&str> = splits.iter().skip(1).collect();
            if fs_item.find(&skipped.join("/")) {
                return true;
            }
        }

        return false;
    }
}