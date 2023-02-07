mod fs;

use fs::{FsItem, File, Folder};

const THRESHOLD: u32 = 100_000;

fn main() {

    let input = std::fs::read_to_string("src/day7/input/input.txt").unwrap();

    let mut pwd = Vec::new();
    let mut tree = Folder::new("/".to_string(), 1);
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
                // dir rbsrpg
                // dir rzgnbgv
                // dir svsgnbs
                let dir_splits: Vec<&str> = line.split(' ').collect();
                let dir_name = dir_splits[1];
                //   check with pwd if already exists
                println!("DIR input");
                match tree.find3(&pwd, Some(dir_name)) {
                    None => {
                        println!("DIR not found pwd: {:?} file: {}", pwd, dir_name);

                        let mut pwd_fs_item = tree.find3(&pwd, None).unwrap();
                        println!("DIR adding...before");
                        pwd_fs_item.print();
                        pwd_fs_item.add(Box::new(Folder::new(dir_name.to_string(), pwd_fs_item.depth() + 1)));
                        println!("DIR adding...after");
                        pwd_fs_item.print();
                    }
                    _ => {
                        println!("DIR found pwd: {:?} file: {}", pwd, dir_name);
                    },
                }

                //   if not - create and add it into struct
            } else {
                println!("FILE input");
                // file
                // 71582 wrqbm
                let file_splits: Vec<&str> = line.split(' ').collect();
                let file_size = file_splits[0].parse::<u32>().unwrap();
                let file_name = file_splits[1];
                //   check with pwd if already exists

                match tree.find3(&pwd, Some(file_name)) {
                    None => {
                        println!("FILE not found pwd: {:?} file: {}", pwd, file_name);
                        let mut pwd_fs_item = tree.find3(&pwd, None).unwrap();
                        println!("FILE adding...before");
                        pwd_fs_item.print();
                        pwd_fs_item.add(Box::new(File::new(file_name.to_string(), file_size, pwd_fs_item.depth()+1)));
                        println!("FILE adding...after");
                        pwd_fs_item.print();
                    }
                    _ => {
                        println!("FILE found pwd: {:?} file: {}", pwd, file_name);
                    },
                }
            }
        }
    }
    println!("************");
    tree.print();
    println!("************");
    println!("tree.disk_usage()={}", tree.disk_usage());

    println!("************");
    println!("tree.tree.get_disk_usage_of_all_folders_less_than(THRESHOLD)={}", tree.get_disk_usage_of_all_folders_less_than(THRESHOLD));
    println!("************");
    //iterate tree and sum all dirs which are above threshold
}
