use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::vec;

const INPUT: &str = include_str!("day7_input.txt");

#[derive(Default, Debug)]
struct Directory {
    path: PathBuf,
    parent: Option<Weak<RefCell<Directory>>>,
    files_size: u32,
    dir_size: u32,
    dirs: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Directory {
    fn iter(d: Rc<RefCell<Self>>) -> DirectoryIter {
        DirectoryIter { stack: vec![d] }
    }
}

struct DirectoryIter {
    stack: Vec<Rc<RefCell<Directory>>>,
}

impl Iterator for DirectoryIter {
    type Item = Rc<RefCell<Directory>>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(d) = self.stack.pop() else {
            return None;
        };

        self.stack
            .append(&mut d.borrow().dirs.values().cloned().collect());

        Some(d)
    }
}

fn fill_directory_size(root: &Rc<RefCell<Directory>>) -> u32 {
    let mut root = root.borrow_mut();
    root.dir_size = root.files_size;

    let dirs: Vec<_> = root.dirs.values().cloned().collect();
    for sub in dirs {
        root.dir_size += fill_directory_size(&sub);
    }

    root.dir_size
}

fn parse(input: &str) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory {
        path: Path::new("/").into(),
        ..Default::default()
    }));

    let mut cwd = root.clone();

    for l in input.lines() {
        match &l[0..4] {
            "$ cd" => {
                let p = &l[5..];

                match p {
                    "/" => {}
                    ".." => {
                        let parent = cwd.borrow().parent.as_ref().unwrap().upgrade().unwrap();
                        cwd = parent;
                    }
                    _ => {
                        let subdir = cwd.borrow().dirs.get(p).unwrap().clone();
                        cwd = subdir;
                    }
                }
            }
            "$ ls" => {}
            _ => {
                let (first, name) = l.split_once(' ').unwrap();

                if first == "dir" {
                    let parent = Rc::downgrade(&cwd);

                    let mut cwd = cwd.borrow_mut();
                    let path = cwd.path.join(name);

                    cwd.dirs.insert(
                        name.into(),
                        Rc::new(RefCell::new(Directory {
                            path,
                            parent: Some(parent),
                            ..Default::default()
                        })),
                    );
                } else {
                    let mut cwd = cwd.borrow_mut();
                    cwd.files_size += first.parse::<u32>().unwrap();
                }
            }
        }
    }

    fill_directory_size(&root);

    root
}

fn solve_part1(input: &str) -> u32 {
    let root = parse(input);

    Directory::iter(root)
        .map(|d| d.borrow().dir_size)
        .filter(|&s| s <= 100000)
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let root = parse(input);
    let to_be_freed = root.borrow().dir_size - 40000000;

    Directory::iter(root)
        .map(|d| d.borrow().dir_size)
        .filter(|&s| s >= to_be_freed)
        .min()
        .unwrap()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7() {
        const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        assert_eq!(solve_part1(TEST_INPUT), 95437);
        assert_eq!(solve_part1(INPUT), 1667443);

        assert_eq!(solve_part2(TEST_INPUT), 24933642);
        assert_eq!(solve_part2(INPUT), 8998590);
    }
}
