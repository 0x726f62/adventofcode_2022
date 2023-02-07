mod file;
mod folder;

pub use file::File;
pub use folder::Folder;

pub trait FsItem {
    fn disk_usage(&self) -> u32;
    fn disk_usage_with_threshold(&self, threshold: u32) -> u32;
    //consider returning FsItem
    fn find(&mut self, keyword: &str) -> Option<Box<&mut dyn FsItem>>;
    // fn find2(&mut self, path: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>>;
    fn find3(&mut self, path: &[&str], item: Option<&str>) -> Option<Box<&mut dyn FsItem>>;

    fn print(&self);

    fn depth(&self) -> u32;

    fn add(&mut self, fs_item: Box<dyn FsItem>);
    fn get_name(&self) -> &str;
    fn is_folder(&self) -> bool;
}