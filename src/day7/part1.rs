mod fs;

use fs::{FsItem, File, Folder};

fn main() {

    let input = std::fs::read_to_string("src/day7/input/input.txt").unwrap();

    let mut pwd = Vec::new();
    let mut tree = Folder::new("/", 1);
    pwd.push("/");
    let mut pwd_fsItem = tree.find("/").unwrap();

    for line in input.lines().skip(1) {
        //if cmd
        if line.starts_with("$") {
            // if ls -> next iter
            if line.eq("$ ls") {
                continue
            }

            // if cd
            if line.starts_with("$ cd") {

                if line.eq("$ cd ..") {
                    //   .. - pop from pwd
                    pwd.pop();
                    pwd_fsItem = tree.find(&pwd.join("/")).unwrap();
                } else {
                    //   x - push into pwd
                    let cd_splits: Vec<&str> = line.split(' ').collect();
                    pwd.push(cd_splits[2]);
                    pwd_fsItem = tree.find(&pwd.join("/")).unwrap();
                }
            }

        } else {
            //if not cmd

            // folder
            if line.starts_with("dir") {
                let dir_splits: Vec<&str> = line.split(' ').collect();
                let dir_name = dir_splits[1];
                //   check with pwd if already exists
                let mut dir_path = pwd.join("/");
                dir_path.push_str(dir_name);
                match tree.find(&dir_path) {
                    None => {
                        let depth = pwd_fsItem.depth();
                        pwd_fsItem.add(Folder::new(dir_name, pwd_fsItem.depth() + 1));
                    }
                    _ => (),
                }

                //   if not - create and add it into struct
            } else {
                // file
                let file_splits: Vec<&str> = line.split(' ').collect();
                let file_size = file_splits[0].parse::<u32>().unwrap();
                let file_name = file_splits[1];
                //   check with pwd if already exists
                let mut file_path = pwd.join("/");
                file_path.push_str(file_name);
                //   if not - create File with size and add it into struct
                match tree.find(&file_path) {
                    None => {
                        pwd_fsItem.add(File::new(file_name, file_size, pwd_fsItem.depth()+1));
                    }
                    _ => (),
                }
            }
        }
    }

    //iterate tree and sum all dirs which are above threshold
}
