use std::fs::{File, OpenOptions};

mod vm_translator;

fn main() {
    println!("Hello, world!");

    vm_translator::translate_file(
        File::open("./data/project7/BasicTest.vm").unwrap(),
        OpenOptions::new().create(true).write(true).open("./data/project7/BasicTest.vm").unwrap(),
    );
}
