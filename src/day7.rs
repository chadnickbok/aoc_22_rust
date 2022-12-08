use crate::utils;
use std::{str::FromStr, collections::VecDeque};

#[derive(Clone)]
struct AocFile {
    name: String,
    size: usize,
}

#[derive(Clone)]
struct AocDir {
    name: String,
    subdirs: Vec<Box<AocDir>>,
    files: Vec<AocFile>,
}

impl AocFile {
    fn new(name: String, size: usize) -> AocFile {
        AocFile{size, name}
    }
}

impl AocDir {
    fn new(name: String) -> AocDir {
        return AocDir{
            name,
            subdirs: Vec::new(),
            files: Vec::new(),
        };
    }

    fn insert_dir(&mut self, path: &Vec<String>, depth: usize, dirname: String) {
        if path.len() == depth {
            self.subdirs.push(Box::new(AocDir::new(dirname)));
        } else {
            let mut dir_i = 999;
            let mut found = false;
            for i in 0..self.subdirs.len() {
                if self.subdirs[i].name == path[depth] {
                    dir_i = i;
                    found = true;
                    break;
                }
            }
            if !found {
                println!("Did not find dir {}", dirname);
            }
            self.subdirs[dir_i].insert_dir(path, depth + 1, dirname);
        }
    }

    fn insert_file(&mut self, path: &Vec<String>, depth: usize, filename: String, filesize: usize) {
        if path.len() == depth {
            self.files.push(AocFile::new(filename, filesize));
        } else {
            let mut dir_i = 999;
            for i in 0..self.subdirs.len() {
                if self.subdirs[i].name == path[depth] {
                    dir_i = i;
                    break;
                }
            }
            self.subdirs[dir_i].insert_file(path, depth + 1, filename, filesize);
        }
    }

    fn get_size(&self) -> usize {
        let mut total = 0;

        for subdir in self.subdirs.iter() {
            total += subdir.get_size();
        }

        for subfile in self.files.iter() {
            total += subfile.size;
        }

        return total;
    }

    fn gather_sizes(&self, result: &mut Vec<usize>) {
        let cur_size = self.get_size();
        result.push(cur_size);

        for subdir in self.subdirs.iter() {
            subdir.gather_sizes(result);
        }
    }
}

pub fn star1(filename: &str) -> Result<usize, utils::AocError> {
    let lines = utils::read_lines(filename).expect("failed to read lines from file");
    let total = 0;

    let mut root = AocDir::new("/".to_string());
    let mut hierarchy: Vec<String> = Vec::new();

    for raw_line in lines {
        let mut cur_dir: &mut AocDir = &mut root;

        let line = raw_line.expect("oops bad line");
        let cmd: Vec<&str> = line.split_whitespace().collect();
        if cmd[0] == "$" {
            if cmd[1] == "cd" {
                match cmd[2] {
                    "/" => {
                        println!("CD ROOT");
                        continue;
                    },
                    ".." => {
                        println!("Go up a dir");
                        hierarchy.pop();
                    },
                    _ => {
                        println!("Cd into dir: {}", cmd[2]);
                        hierarchy.push(cmd[2].to_string());
                        println!("PWD {:?}", hierarchy);
                    }
                }
            }
        } else if cmd[0] == "dir" {
            println!("Inserting dir: {}", cmd[1]);
            root.insert_dir(&hierarchy, 0, cmd[1].to_string());
        } else {
            let filesize = usize::from_str(cmd[0]).expect("invalid count");
            println!("file");
            root.insert_file(&hierarchy, 0, cmd[1].to_string(), filesize);
        }
    }

    let total_filespace = 70000000;
    let used_filespace = root.get_size();

    println!("currently used is {}", used_filespace);

    let unused_target = 30000000;
    let currently_unused = total_filespace - used_filespace; 

    let size_target = unused_target - currently_unused;

    println!("currently unused is {}", currently_unused);
    println!("target is {}", size_target);

    let mut smallest = root.get_size();
    let mut sizes = Vec::new();

    root.gather_sizes(&mut sizes);

    for size in sizes {
        if (size > size_target) && (size < smallest) {
            smallest = size;
        }
    }
    
    Ok(smallest)
}