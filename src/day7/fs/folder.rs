use super::FsItem;

static PATH_SEP: &'static str = "/";

pub struct Folder<'a> {
    name: &'a str,
    fs_items: Vec<Box<dyn FsItem + 'a>>,
}

impl<'a>  Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            fs_items: vec![],
        }
    }

    pub fn add(&mut self, fs_item: impl FsItem + 'a) {
        self.fs_items.push(Box::new(fs_item));
    }
}

impl<'a> FsItem for Folder<'a> {
    fn disk_usage(&self) -> u32 {
        let mut sum: u32 = 0;
        for fs_item in self.fs_items.iter() {
            sum += fs_item.disk_usage();
        }

        sum
    }

    fn find(&self, keyword: &str) -> bool {
        //take the first
        let splits: Vec<&str> = keyword.split(PATH_SEP).collect();
        // compare with name
        if !self.name.eq(splits[0]) {
            // no point to look further in this sub tree
            return false;
        } else if splits.len() == 1 {
            //also case to check a for folder existence
            return true;
        }

        //TODO refactor
        let mut splits_without_first_level: Vec<&str> = vec![];
        for index in 1..(splits.len()-1) {
            splits_without_first_level.push(splits[index]);
        }

        for fs_item in self.fs_items.iter() {
            if fs_item.find(&(splits_without_first_level.join(PATH_SEP))) {
                //match found in sub tree
                return true;
            }
        }

        return false;
    }
}