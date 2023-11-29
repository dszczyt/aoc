use std::{cell::RefCell, rc::Rc};

pub struct Shell {
    pub cwd: Vec<String>,
}

impl Shell {}

pub trait Command {
    fn run(&self, file: Rc<RefCell<Box<File>>>) -> Rc<RefCell<Box<File>>>;
    fn interpret_output(&self, cwd: Rc<RefCell<Box<File>>>, input: &str);
}

#[derive(Debug)]
pub struct Cd {
    pub dir: String,
}

impl From<&str> for Cd {
    fn from(input: &str) -> Self {
        Self {
            dir: input.to_owned(),
        }
    }
}

impl Command for Cd {
    fn run(&self, file: Rc<RefCell<Box<File>>>) -> Rc<RefCell<Box<File>>> {
        if self.dir == ".." {
            return file.borrow().parent.clone().unwrap();
        }

        if self.dir == "/" {
            return file;
            // return file.borrow().root.clone().unwrap();
            // let mut result = file.borrow();
            // while let Some(parent) = result.parent.unwrap().borrow() {
            //     result = parent;
            // }
            // return result.clone();
        }

        match file
            .borrow()
            .children
            .iter()
            .find(|f| f.borrow().name == self.dir)
        {
            Some(f) => match f.borrow().is_dir {
                true => f.clone(),
                false => unreachable!(),
            },
            None => {
                unreachable!()
            }
        }
    }

    fn interpret_output(&self, _: Rc<RefCell<Box<File>>>, _: &str) {
        unreachable!()
    }
}

pub struct Ls {}

impl Command for Ls {
    fn run(&self, file: Rc<RefCell<Box<File>>>) -> Rc<RefCell<Box<File>>> {
        file.clone()
    }

    fn interpret_output(&self, cwd: Rc<RefCell<Box<File>>>, input: &str) {
        let (left, right) = input.split_once(' ').unwrap();
        if cwd
            .clone()
            .borrow()
            .children
            .clone()
            .into_iter()
            .any(|child| child.borrow().name == input.to_owned())
        {
            return;
        }
        let f;
        if left == "dir" {
            f = Rc::new(RefCell::new(Box::new(File {
                root: cwd.borrow().root.clone(),
                children: vec![],
                parent: Some(cwd.clone()),
                is_dir: true,
                name: right.to_owned(),
                size: None,
            })));
        } else {
            f = Rc::new(RefCell::new(Box::new(File {
                root: cwd.borrow().root.clone(),
                children: vec![],
                parent: Some(cwd.clone()),
                is_dir: false,
                name: right.to_owned(),
                size: Some(usize::from_str_radix(left, 10).unwrap()),
            })));
        }
        // dbg!(&f);
        cwd.borrow_mut().children.push(f);
    }
}

pub fn read_terminal_output(input: &str) -> Rc<RefCell<Box<File>>> {
    let root = Rc::new(RefCell::new(Box::new(File::root())));

    let mut cwd = root.clone();

    let mut current_cmd: Option<Box<dyn Command>> = None;

    input.lines().for_each(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        match left {
            "$" => {
                let command: Vec<&str> = right.splitn(2, ' ').collect();
                match command.get(0).unwrap().as_ref() {
                    "cd" => {
                        let dir: &str = command.get(1).unwrap().as_ref();
                        let cd = Cd {
                            dir: dir.to_owned(),
                        };
                        current_cmd = Some(Box::new(cd));
                    }
                    "ls" => current_cmd = Some(Box::new(Ls {})),
                    _ => unreachable!(),
                }
                cwd = current_cmd.as_ref().unwrap().run(cwd.clone());
            }
            _ => {
                current_cmd
                    .as_ref()
                    .unwrap()
                    .interpret_output(cwd.clone(), line);
            }
        }
    });

    root
}

#[derive(Clone, Debug)]
pub struct File {
    pub root: Option<Rc<RefCell<Box<File>>>>,
    pub children: Vec<Rc<RefCell<Box<File>>>>,
    pub parent: Option<Rc<RefCell<Box<File>>>>,
    pub is_dir: bool,
    pub name: String,
    pub size: Option<usize>,
}

impl File {
    pub fn root() -> Self {
        let root = Self {
            root: None,
            children: vec![],
            parent: None,
            is_dir: true,
            name: "".to_owned(),
            size: None,
        };
        // root.root = Some(Rc::new(RefCell::new(Box::new(root))));
        root
    }
    pub fn size(&self) -> usize {
        if self.is_dir {
            self.children
                .iter()
                .map(|child| child.borrow().size())
                .sum()
        } else {
            self.size.unwrap()
        }
    }

    pub fn sum_of_dir_sizes(&self, max: usize) -> usize {
        let children_dir = self.children.iter().filter(|child| child.borrow().is_dir);
        children_dir
            .clone()
            .map(|child| {
                let size = child.borrow().size();
                if size < max {
                    return size;
                }
                0
            })
            .sum::<usize>()
            + children_dir
                .map(|child| child.borrow().sum_of_dir_sizes(max))
                .sum::<usize>()
    }

    pub fn search(&self) -> Vec<usize> {
        let mut dirs: Vec<usize> = vec![];

        let children_dir = self.children.iter().filter(|child| child.borrow().is_dir);
        children_dir.clone().for_each(|child| {
            let size = child.borrow().size();

            dirs.push(size);
        });
        let mut subdirs: Vec<usize> = children_dir
            .clone()
            .map(|dir| dir.borrow().search())
            .flatten()
            .collect();
        dirs.append(&mut subdirs);
        dirs.sort();
        dirs
    }
}

#[cfg(test)]
mod test {
    use crate::read_terminal_output;

    #[test]
    fn example() {
        let input = include_str!("example");
        let root = read_terminal_output(input);
        let sum_of_dir_sizes = root.borrow().sum_of_dir_sizes(100000);
        assert_eq!(sum_of_dir_sizes, 95437)
    }

    #[test]
    fn part2() {
        let input = include_str!("example");
        let root = read_terminal_output(input);
        let root_size = root.borrow().size();
        let actual_free = 70000000 - root_size;
        let to_free = 30000000 - actual_free;
        assert_eq!(to_free, 8381165);
        let sizes = root.borrow().search();
        assert_eq!(
            sizes
                .iter()
                .filter(|n| n.clone().clone() > to_free)
                .nth(0)
                .unwrap()
                .clone(),
            24933642
        )
    }
}

fn main() {
    let input = include_str!("input");
    let root = read_terminal_output(input);
    let sum_of_dir_sizes = root.borrow().sum_of_dir_sizes(100000);
    dbg!(&sum_of_dir_sizes);

    let root_size = root.borrow().size();
    dbg!(&root_size);
    let actual_free = 70000000 - root_size;
    let to_free = 30000000 - actual_free;
    dbg!(&to_free);
    let sizes = root.borrow().search();

    dbg!(sizes
        .iter()
        .filter(|n| n.clone().clone() > to_free)
        .nth(0)
        .unwrap()
        .clone());
}
