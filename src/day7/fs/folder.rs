use crate::THRESHOLD;
use super::FsItem;

static PATH_SEP: &'static str = "/";

pub struct Folder {
    name: String,
    depth: u32,
    fs_items: Vec<Box<dyn FsItem>>,
}

impl Folder {
    pub fn new(name: String, depth: u32) -> Self {
        Self {
            name,
            depth,
            fs_items: vec![],
        }
    }

    pub fn get_disk_usage_of_all_folders_less_than(&self, threshold: u32) -> u32 {
        let mut sum :u32 = 0;
        for fs_item in self.fs_items.iter() {
            sum += fs_item.disk_usage_with_threshold(threshold);
        }
        sum
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

    fn find(&mut self, keyword: &str) -> Option<Box<&mut dyn FsItem>> {
        //take the first
        println!("find str={}", keyword);
        let splits: Vec<&str> = keyword.split(PATH_SEP).collect();
        // compare with name
        if !self.name.eq(splits[0]) {
            // no point to look further in this sub tree
            return None;
        } else if splits.len() == 1 {
            //also case to check a for folder existence
            return Some(Box::new(self));
        }

        //TODO refactor
        let mut splits_without_first_level: Vec<&str> = vec![];
        for index in 1..(splits.len()-1) {
            splits_without_first_level.push(splits[index]);
        }

        for fs_item in self.fs_items.iter_mut() {
            match fs_item.find(&(splits_without_first_level.join(PATH_SEP))) {
                //match found in sub tree
                Some(x) => return Some(x),
                None => continue,
            }
        }

        return None;
    }

    // fn find2(&mut self, path: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>> {
    //     println!("---------------");
    //     println!("find2 self name={}", self.name);
    //     println!("find2 str={:?}", item);
    //     println!("find2 path={:?}", path);
    //
    //     if let Some(i) = item {
    //         if path.is_empty() {
    //             return None;
    //         } else {
    //             if !self.name.eq(path[0]) {
    //                 return None;
    //             }
    //             for fs_item in self.fs_items.iter_mut() {
    //                 if fs_item.get_name().eq(path[1]) {
    //                     match fs_item.find2(&path[1..], item) {
    //                         //match found in sub tree
    //                         Some(x) => return Some(x),
    //                         None => continue,
    //                     }
    //                 }
    //
    //             }
    //             return None;
    //         }
    //     } else {
    //         if path.len() == 1 {
    //             if path[0] == self.name {
    //                 return Some(Box::new(self));
    //             } else {
    //                 return None;
    //             }
    //         } else {
    //             if !self.name.eq(path[0]) {
    //                 return None;
    //             }
    //             for fs_item in self.fs_items.iter_mut() {
    //                 if fs_item.get_name().eq(path[1]) {
    //                     match fs_item.find2(&path[1..], item) {
    //                         //match found in sub tree
    //                         Some(x) => return Some(x),
    //                         None => continue,
    //                     }
    //                 }
    //
    //             }
    //         }
    //     }
    //     return None;
    // }

    fn find3(&mut self, path: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>> {
        println!("---------------");
        println!("find3 self name={}", self.name);
        println!("find3 str={:?}", item);
        println!("find3 path={:?}", path);

        if let Some(i) = item {
            if path.is_empty() {
                if self.name.eq(i) {
                    return Some(Box::new(self));
                } else {
                    return None;
                }

            } else {
                if !self.name.eq(path[0]) {
                    return None;
                }

                for fs_item in self.fs_items.iter_mut() {
                    if path.len() > 1 {
                        if !fs_item.get_name().eq(path[1]) {
                            continue
                        }

                    }

                    match fs_item.find3(&path[1..], item) {
                        //match found in sub tree
                        Some(x) => return Some(x),
                        None => continue,
                    }
                }
                return None;
            }

        } else {
            if path.len() == 1 {
                if self.name.eq(path[0]) {
                    return Some(Box::new(self));
                } else {
                    return None;
                }
            } else {
                if !self.name.eq(path[0]) {
                    return None;
                }
                for fs_item in self.fs_items.iter_mut() {
                    if path.len() > 1 {
                        if !fs_item.get_name().eq(path[1]) {
                            continue
                        }

                    }
                    match fs_item.find3(&path[1..], item) {
                        //match found in sub tree
                        Some(x) => return Some(x),
                        None => continue,
                    }
                }
            }
        }
        return None;
    }

    fn print(&self) {
        for _ in 0..self.depth {
            print!("-");
        }
        println!(" FOLDER {}", self.name);
        println!(" number of subItems {}", self.fs_items.len());

        for fs_item in self.fs_items.iter() {
            fs_item.print();
        }
    }

    fn depth(&self) -> u32 {
        self.depth
    }

    fn add(&mut self, fs_item: Box<dyn FsItem>) {
        self.fs_items.push(fs_item);
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn disk_usage_with_threshold(&self, threshold: u32) -> u32 {
        let mut folders_sum_under_threshold:u32 = 0;
        let mut files_sum:u32 = 0;

        for fs_item in self.fs_items.iter() {
            let sub_sum = fs_item.disk_usage_with_threshold(threshold);
            if fs_item.is_folder() {
                if sub_sum <= threshold {
                    folders_sum_under_threshold += sub_sum;
                }
            } else {
                files_sum +=sub_sum;
            }
        }

        if files_sum + folders_sum_under_threshold <= THRESHOLD {
            files_sum + folders_sum_under_threshold
        } else {
            folders_sum_under_threshold
        }
    }

    fn is_folder(&self) -> bool {
        true
    }
}