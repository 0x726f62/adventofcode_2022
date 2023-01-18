mod fs;

use fs::{FsItem, File, Folder};

fn main() {

    let input = std::fs::read_to_string("src/day7/input/input.txt").unwrap();

    let mut pwd = Vec::new();
    let root = Folder::new("/");
    pwd.push("/");

    for line in input.lines().skip(1) {
        //if ls
        // file
        //   check with pwd if already exists
        //   if not - create File with size and add it into struct
        // folder
        //   check with pwd if already exists
        //   if not - create and add it into struct

        //if cd
        //   x - push into pwd
        //   .. - pop from pwd
        //parse the input
        //cd only over 1 level -> so just stack for pwd is sufficient
    }

    //iterate tree and sum all dirs which are above threshold
}
