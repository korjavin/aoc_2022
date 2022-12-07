#![allow(dead_code)]
use std::io::{self, BufRead};

// Structure to hold directory name and all its files and subdirectories
// #[derive( Clone)]
struct Dir<'a> {
    name: String,
    files: Vec<File>,
    subdirs: Vec<Dir<'a>>,
    parent: Option<Box<&'a Dir<'a>>>,
}

impl<'a> Dir<'a> {
    fn new(name: String, dir: Option<Box<&'a Dir>>) ->  &'static Dir<'a> {
        &Dir {
            name: name,
            files: Vec::new(),
            subdirs: Vec::new(),
            parent: dir,
        }
    }
    // function to add a file to Dir
    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
    // function to add a Dir to Dir
    fn add_dir(&'a mut self, dir: Dir<'a>) {
        self.subdirs.push(dir);
    }
    // function to calculate sum of size of all files in Dir
    fn size(&self) -> u64 {
        let mut size = 0;
        for file in &self.files {
            size += file.size;
        }
        for dir in &self.subdirs {
            size += dir.size();
        }
        size
    }
}

#[test]
fn test_dir_size() {
    let mut dir = Dir::new("test".to_string(), None::<Box<&Dir>>);
    dir.add_file(File::new("test1".to_string(), 10));
    dir.add_file(File::new("test2".to_string(), 20));
    dir.add_file(File::new("test3".to_string(), 30));
    assert_eq!(dir.size(), 60);
}

fn test_subdir_size() {
    let mut dir = Dir::new("test".to_string(), None::<Box<&Dir>>);
    dir.add_file(File::new("test1".to_string(), 10));
    let mut subdir = Dir::new("test".to_string(), Some(Box::new(&dir)));
    subdir.add_file(File::new("test2".to_string(), 20));
    subdir.add_file(File::new("test3".to_string(), 30));
    assert_eq!(subdir.size(), 50);
    assert_eq!(dir.size(), 60);
}

// Structure to hold filename and its size
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> File {
        File {
            name: name,
            size: size,
        }
    }
}

fn main() {
    let mut currdir = Some(Box::new(Dir::new("root".to_string(), None::<Box<&Dir>>)));
    for line in io::stdin().lock().lines() {

        // match if line content is started from $ cd then create a new Dir and set it as subdir to
        // current dir
        if line.unwrap().starts_with("$ cd") {
            // let dir = currdir.add_dir(Dir::new(line.unwrap().split(" ").nth(2).unwrap().to_string(), currdir));
        }
        

    }

    println!("Hello, world!");
}
