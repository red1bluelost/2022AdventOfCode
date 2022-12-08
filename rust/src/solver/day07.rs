use std::cell::RefCell;
use std::io;
use std::rc::{Rc, Weak};
use std::str::FromStr;

use crate::problem::Solution;

#[derive(Debug)]
enum ShellLine {
    CD(String),
    LS,
    DIR(String),
    FILE(usize, String),
}

#[derive(Debug)]
enum DirInner {
    File,
    Dir(RefCell<Vec<Rc<Directory>>>),
}

#[derive(Debug)]
struct Directory {
    parent: Weak<Directory>,
    name: String,
    size: RefCell<usize>,
    inner: DirInner,
}

impl Directory {
    fn base(&self) -> Option<Rc<Directory>> {
        let mut cur_dir = match self.parent.upgrade() {
            None => return None,
            Some(rc) => rc,
        };

        loop {
            match cur_dir.parent.upgrade() {
                None => return Some(cur_dir),
                Some(rc) => cur_dir = rc,
            }
        }
    }

    fn add_size_to_dir(&self, size: usize) {
        assert!(self.is_dir());
        *self.size.borrow_mut() += size;
        let mut cur_dir = match self.parent.upgrade() {
            None => return,
            Some(rc) => rc,
        };
        loop {
            *cur_dir.size.borrow_mut() += size;
            match cur_dir.parent.upgrade() {
                None => return,
                Some(rc) => cur_dir = rc,
            }
        }
    }

    fn dir_iter(&self) -> Option<DirIter> {
        if self.is_file() {
            return None;
        } else {
            Some(DirIter { stack: vec![&self] })
        }
    }

    fn is_dir(&self) -> bool {
        matches!(self.inner, DirInner::Dir(_))
    }

    fn is_file(&self) -> bool {
        matches!(self.inner, DirInner::File)
    }

    fn find_dir(&self, name: &str) -> Option<Rc<Directory>> {
        self.files_and_dirs().map(|v| {
            v.borrow()
                .iter()
                .filter(|fd| fd.is_dir())
                .find(|d| d.name == name)
                .unwrap()
                .clone()
        })
    }

    fn files_and_dirs(&self) -> Option<&RefCell<Vec<Rc<Directory>>>> {
        match &self.inner {
            DirInner::File => None,
            DirInner::Dir(files) => Some(files),
        }
    }

    fn build(cli_iter: impl Iterator<Item = ShellLine>) -> Rc<Directory> {
        let mut cur_dir = Rc::new(Directory {
            parent: Default::default(),
            name: "/".to_string(),
            size: Default::default(),
            inner: DirInner::Dir(Default::default()),
        });

        let base = Rc::clone(&cur_dir);

        for sl in cli_iter {
            match sl {
                ShellLine::CD(s) if s == ".." => cur_dir = cur_dir.parent.upgrade().unwrap(),
                ShellLine::CD(s) if s == "/" => cur_dir = cur_dir.base().unwrap_or(cur_dir),
                ShellLine::CD(dir) => cur_dir = cur_dir.find_dir(&dir).unwrap(),
                ShellLine::LS => { /* intentionally no-op */ }
                ShellLine::DIR(name) => {
                    let new_dir = Rc::new(Directory {
                        parent: Rc::downgrade(&cur_dir),
                        name,
                        size: Default::default(),
                        inner: DirInner::Dir(Default::default()),
                    });
                    cur_dir.files_and_dirs().unwrap().borrow_mut().push(new_dir);
                }
                ShellLine::FILE(size, name) => {
                    cur_dir.add_size_to_dir(size);
                    let new_file = Rc::new(Directory {
                        parent: Rc::downgrade(&cur_dir),
                        name,
                        size: RefCell::new(size),
                        inner: DirInner::File,
                    });
                    cur_dir
                        .files_and_dirs()
                        .unwrap()
                        .borrow_mut()
                        .push(new_file);
                }
            }
        }

        base
    }
}

struct DirIter<'a> {
    stack: Vec<&'a Directory>,
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_dir = self.stack.pop()?;
        cur_dir
            .files_and_dirs()
            .unwrap()
            .borrow()
            .iter()
            .filter(|fd| fd.is_dir())
            .for_each(|d| self.stack.push(unsafe { &*Rc::as_ptr(&d) }));
        Some(cur_dir)
    }
}

impl FromStr for ShellLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ShellLine::*;
        Ok(match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            &["$", "ls"] => LS,
            &["$", "cd", dir_name] => CD(dir_name.to_string()),
            &["dir", dir_name] => DIR(dir_name.to_string()),
            &[size_s, file_name] => {
                if let Ok(size) = size_s.parse() {
                    FILE(size, file_name.to_string())
                } else {
                    Err(format!("unknown two string line: {} {}", size_s, file_name))?
                }
            }
            l => Err(format!("unknown line: {:?}", l))?,
        })
    }
}

pub fn solve(r: impl io::BufRead) -> io::Result<Solution> {
    let file_sys = Directory::build(r.lines().map(|l| l.unwrap().parse().unwrap()));

    let sizes: Vec<usize> = file_sys
        .dir_iter()
        .unwrap()
        .map(|d| *d.size.borrow())
        .collect();

    let part1 = sizes
        .iter()
        .filter(|&&s| s <= 100_000)
        .sum::<usize>()
        .to_string();

    let space_needed = 30_000_000 - (70_000_000 - *file_sys.size.borrow());
    let part2 = sizes
        .iter()
        .filter(|&&s| s >= space_needed)
        .min()
        .unwrap()
        .to_string();
    Ok(Solution { part1, part2 })
}
