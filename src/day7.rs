use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct File {
    parent: Option<Rc<RefCell<File>>>,
    children: Vec<Rc<RefCell<File>>>,
    name: String,
    size: Option<usize>,
}

impl File {
    fn new(name: &str, size: Option<usize>) -> Self {
        File {
            parent: None,
            children: vec![],
            name: name.to_owned(),
            size,
        }
    }

    fn get_dir_size(&self) -> usize {
        match self.size {
            Some(size) => size,
            None => self.children.iter().fold(0usize, |acc, child| {
                if child.borrow().children.len() == 0 {
                    return acc + child.borrow().size.expect("All files should have sizes");
                }
                acc + child.borrow().get_dir_size()
            }),
        }
    }

    fn get_dir_size_below(&self, min_size: usize) -> usize {
        self.children
            .iter()
            .filter(|child| child.borrow().children.len() > 0)
            .fold(0usize, |acc, folder| {
                let folder_size = folder.borrow().size.unwrap();
                let folder_size = if folder_size <= min_size {
                    folder_size
                } else {
                    0
                };
                acc + folder_size + folder.borrow().get_dir_size_below(min_size)
            })
    }

    fn get_smallest_folder_size_above(&self, unused_space: usize) -> usize {
        let disk_space_needed = 30_000_000 - unused_space;
        self.children
            .iter()
            .filter(|child| {
                child.borrow().children.len() > 0
                    && child.borrow().size.unwrap() >= disk_space_needed
            })
            .map(|folder| {
                let folder_size = folder.borrow().size.unwrap();
                let folder_children_size =
                    folder.borrow().get_smallest_folder_size_above(unused_space);
                if folder_children_size < folder_size {
                    return folder_children_size;
                } else {
                    return folder_size;
                }
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    fn populate_dir_sizes(&mut self) {
        if self.size.is_some() {
            return;
        }
        for child in self.children.iter() {
            child.borrow_mut().populate_dir_sizes();
        }
        self.size = Some(self.get_dir_size());
    }

    #[allow(dead_code)]
    fn get_parents_depth(&self) -> usize {
        if self.parent.is_none() {
            return 1;
        }
        self.parent.as_ref().unwrap().borrow().get_parents_depth() + 1
    }

    #[allow(dead_code)]
    fn print(&self, depth: usize) -> String {
        let size = match &self.size {
            Some(size) => size.to_string().to_owned(),
            None => "Unknown".to_owned(),
        };

        return String::from("  ").repeat(depth)
            + "- "
            + &self.name
            + " "
            + &size
            + "\n"
            + &self
                .children
                .iter()
                .map(|child| child.borrow().print(child.borrow().get_parents_depth()))
                .collect::<Vec<String>>()
                .join("");
    }
}

pub fn solution() {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();
    let root = Rc::new(RefCell::new(File::new("/", None)));
    let mut current_dir = Rc::clone(&root);
    for line in input.trim().split("\n").skip(1) {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 3 {
            let directory_change = parts[2];
            if directory_change == ".." {
                let current_clone = Rc::clone(&current_dir);
                current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                continue;
            }
            if directory_change == "/" {
                current_dir = Rc::clone(&root);
                continue;
            }
            let child = Rc::new(RefCell::new(File::new(directory_change, None)));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current_dir));
            }
            current_dir.borrow_mut().children.push(Rc::clone(&child));
            current_dir = child;
            continue;
        }
        if parts[1] == "ls" {
            continue;
        }
        if parts[0] != "dir" {
            let size = Some(parts[0].parse::<usize>().expect("Should be a number"));
            let name = parts[1].to_owned();
            let child = Rc::new(RefCell::new(File::new(&name, size)));
            current_dir.borrow_mut().children.push(Rc::clone(&child));
            {
                let mut mut_child = child.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current_dir));
            }
            continue;
        }
    }
    let max_size = 100_000;
    root.borrow_mut().populate_dir_sizes();
    println!(
        "Total sum of directories below {}: {}",
        max_size,
        root.borrow().get_dir_size_below(max_size)
    );
    println!(
        "Smallest directory needed to be deleted: {}",
        root.borrow()
            .get_smallest_folder_size_above(70_000_000 - root.borrow().size.unwrap())
    );
}
