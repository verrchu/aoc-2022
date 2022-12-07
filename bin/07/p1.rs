use std::{cell::RefCell, collections::HashMap, mem::MaybeUninit, rc::Rc};

static INPUT: &str = include_str!("input");

#[derive(Debug)]
struct Dir {
    parent: Option<Rc<RefCell<Dir>>>,
    sub_dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, usize>,
    size: MaybeUninit<usize>,
}

impl Dir {
    fn new() -> Self {
        Self {
            parent: None,
            sub_dirs: HashMap::new(),
            files: HashMap::new(),
            size: MaybeUninit::uninit(),
        }
    }

    fn calculate_size(node: Rc<RefCell<Dir>>) -> usize {
        let mut node = node.borrow_mut();

        let mut size = 0;
        if !node.sub_dirs.is_empty() {
            size += node
                .sub_dirs
                .values()
                .map(|sub_dir| Dir::calculate_size(sub_dir.clone()))
                .sum::<usize>();
        }

        size += node.files.values().sum::<usize>();

        unsafe {
            *(node.size.as_mut_ptr()) = size;
        }

        size
    }

    fn traverse(node: Rc<RefCell<Dir>>, f: &mut impl FnMut(usize)) {
        let node = node.borrow();

        f(unsafe { *(node.size.as_ptr()) });
        node.sub_dirs
            .values()
            .for_each(|sub_dir| Dir::traverse(sub_dir.clone(), f));
    }
}

fn is_cd(line: &str) -> Option<&str> {
    line.strip_prefix("$ cd ")
}

fn is_file(line: &str) -> Option<(&str, usize)> {
    (!line.starts_with("$"))
        .then(|| line.split(" ").collect::<Vec<_>>())
        .and_then(|parts| (parts[0] != "dir").then(|| (parts[1], parts[0].parse().unwrap())))
}

fn main() {
    let root = Rc::new(RefCell::new(Dir::new()));
    let mut cwd = root.clone();

    for line in INPUT.lines() {
        if let Some(dir) = is_cd(line) {
            match dir {
                "/" => cwd = root.clone(),
                ".." => {
                    let parent = cwd.borrow().parent.clone().unwrap();
                    cwd = parent;
                }
                path => {
                    let next = {
                        let mut node = cwd.borrow_mut();
                        node.sub_dirs
                            .entry(path.to_string())
                            .or_insert(Rc::new(RefCell::new(Dir::new())));

                        node.sub_dirs.get(path).unwrap().clone()
                    };

                    next.borrow_mut().parent = Some(cwd.clone());
                    cwd = next;
                }
            }
        } else if let Some((name, size)) = is_file(line) {
            cwd.borrow_mut().files.insert(name.to_string(), size);
        }
    }

    Dir::calculate_size(root.clone());

    let mut result = 0;
    Dir::traverse(root, &mut |size| {
        if size <= 100_000 {
            result += size;
        }
    });

    println!("{result}");
}
