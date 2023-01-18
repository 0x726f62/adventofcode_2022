mod fs;

use fs::{FsItem, File, Folder};

fn main() {

    let input = std::fs::read_to_string("src/day7/input/input.txt").unwrap();

    let mut pwd = Vec::new();
    let mut tree = Folder::new("/");
    pwd.push("/");

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
                } else {
                    //   x - push into pwd
                    let cd_splits: Vec<&str> = line.split(' ').collect();
                    pwd.push(cd_splits[2]);
                }
            }

        } else {
            //if not cmd

            // folder
            if line.starts_with("dir") {
                let dir_splits: Vec<&str> = line.split(' ').collect();
                let dir_name = dir_splits[1];
                //   check with pwd if already exists
                let mut concat_path = pwd.join("/");
                concat_path.push_str(dir_name);
                if !tree.find(&concat_path ) {
                    tree.add(Folder::new(dir_name));
                }
                //   if not - create and add it into struct
            } else {
                // file
                let file_splits: Vec<&str> = line.split(' ').collect();
                let file_size = file_splits[0].parse::<u32>().unwrap();
                let file_name = file_splits[1];
                //   check with pwd if already exists
                let mut concat_path = pwd.join("/");
                concat_path.push_str(file_name);
                //   if not - create File with size and add it into struct
                if !tree.find(&concat_path ) {
                    tree.add(File::new(file_name, file_size));
                }
            }
        }
    }

    //iterate tree and sum all dirs which are above threshold
}
