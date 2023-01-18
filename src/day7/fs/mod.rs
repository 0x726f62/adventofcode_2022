mod file;
mod folder;

pub use file::File;
pub use folder::Folder;

pub trait FsItem {
    fn disk_usage(&self) -> u32;
    //consider returning FsItem
    fn find(&self, keyword: &str) -> bool;
}