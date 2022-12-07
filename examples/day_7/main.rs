/// This one almost kille me with Lifetimes and mutable borrows in a loop.
use std::fs;
use std::io::Read;


#[derive(Debug)]
enum Child<'a> {
    Directory{children: Vec<Child<'a>>, name: &'a str},
    File{name: &'a str, size: u128}
}

impl<'a> Child<'a> {
    pub fn add_child(&mut self, child: Child<'a>) {
        match self {
            Child::Directory { children, name:_ } => {
                children.push(child);
            },
            _ => panic!("What are you doing!")
        }
    }
    pub fn total_size(&self) -> u128 {
        let mut total = 0;
        match self {
            Child::Directory { children, name:_}=>{
                total += children.iter().map(|f| f.total_size()).sum::<u128>();
            },
            Child::File { name: _, size } => {
                total += size;
            }
        }
        total
    }

    pub fn print(&self, level: usize) {
        for _ in 0..=level {
            print!("\t");
        }
        match self {
            Child::Directory { children, name } => {
                println!("- {} (dir, size={})", name, self.total_size());
                children.iter().for_each(|f| f.print(level + 1));
            },
            Child::File { name, size:_ } => {
                println!("- {} (file, size={})", name, self.total_size())
            }
        }
    }

    pub fn first_challenge(&self) -> u128{
        let mut total = 0;
        if self.is_directory(){
            if self.total_size() <= 100000 {
                total += self.total_size();
            }
            if let Child::Directory { children, name:_ } = self {
                for child in children {
                    total += child.first_challenge()
                }
            }
        }
        total
    }

    fn get_candidates(&self, minimum_size: u128) -> Vec<(String, u128)> {
        let mut results = Vec::new();

        if self.is_directory(){
            if let Child::Directory { children, name } = self {
                if self.total_size() >= minimum_size {
                    results.push((String::from(*name), self.total_size()));
                }
                for child in children {
                    results.append(&mut child.get_candidates(minimum_size));
                }
            }
        }
        results
    }

    /// Returns `true` if the child is [`Directory`].
    ///
    /// [`Directory`]: Child::Directory
    #[must_use]
    fn is_directory(&self) -> bool {
        matches!(self, Self::Directory { .. })
    }
}

#[derive(Debug)]
enum TraversalType<'a> {
    Down(&'a str),
    Parent,
    Root
}
#[derive(Debug)]
enum ParseResult<'a> {
    Traversal(TraversalType<'a>),
    ListCommand,
    ListResult(Child<'a>),
    Error
}


struct FileSystem<'a> {
    root: Child<'a>,
    current_dir: String
}

impl<'a> FileSystem<'a> {
    pub fn new() -> Self {
        Self {
            root: Child::Directory { children: Vec::new(), name: "/" },
            current_dir: String::from("/")
        }
    }

    fn traverse_down(&mut self, dir_name: &'a str) {
        if self.current_dir == "/" {
            self.current_dir = format!("/{}", dir_name);
        }else{
            self.current_dir = format!("{}/{}", self.current_dir, dir_name);
        }
    }

    fn traverse_up(&mut self) {
        let count = self.current_dir.split('/').count();
        self.current_dir = self.current_dir.split('/').take(count-1).collect::<Vec<&str>>().join("/");
        if self.current_dir.is_empty() { self.current_dir = String::from("/"); }
    }

    fn get_current_dir_mut(&mut self) -> &'a mut Child<'a>{
        let mut current = &mut self.root as *mut Child;
        for dir_name in self.current_dir.split('/') {
            if !dir_name.is_empty() {
                if let Child::Directory{children,name:_} = unsafe{&mut *current} {
                    for child in children.iter_mut() {
                        if matches!(child, Child::Directory { children:_, name } if *name == dir_name) {
                            current = child as *mut Child;
                        }
                    }
                }
            }
        }
        unsafe {&mut *current}
    }

    fn add_child(&mut self, child: Child<'a>){       
        self.get_current_dir_mut().add_child(child);
    }

    fn print(&self){
        self.root.print(0);
    }

    fn get_first_challenge(&self){
        println!("The total is {}", self.root.first_challenge());
    }

    fn get_second_challenge(&self){
        let total_disk_space = 70000000u128;
        let needed_space = 30000000u128;
        let unused_space = total_disk_space - self.root.total_size();
        println!("We now have {} unused space, weed need to make the diference: {}", unused_space, needed_space - unused_space);
        let candidates = self.root.get_candidates(needed_space - unused_space);
        println!("{:?}, minium is: {:?}", candidates, candidates.iter().map(|f| f.1).min());
    }
    
}


fn parse_line(line: &str) -> ParseResult{
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    match parts[0] {
        "$" => {
            match parts[1] {
                "ls" => ParseResult::ListCommand,
                "cd" => {
                    match parts[2] {
                        "" => ParseResult::Error,
                        "/" => ParseResult::Traversal(TraversalType::Root),
                        ".." => ParseResult::Traversal(TraversalType::Parent),
                        dir => ParseResult::Traversal(TraversalType::Down(dir)),
                    }
                },
                _ => ParseResult::Error
            }
        }
        other => {
            match other {
                "dir" => ParseResult::ListResult(Child::Directory{children: Vec::new(), name: parts[1] }),
                size => ParseResult::ListResult(Child::File{name: parts[1], size: size.parse::<u128>().unwrap()})
            }
        }
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_7/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let mut file_system=FileSystem::new();
            for line in contents.split("\r\n") {
                match parse_line(line) {
                    ParseResult::ListCommand => {},
                    ParseResult::Traversal(traversal_type) => {
                        match traversal_type {
                            TraversalType::Down(dir_name) => {
                                file_system.traverse_down(dir_name);
                            },
                            TraversalType::Root => {
                                file_system = FileSystem::new();
                            },
                            TraversalType::Parent => {
                                file_system.traverse_up();
                            }
                        }
                    }
                    ParseResult::ListResult(result) => {
                        file_system.add_child(result);
                    }
                    ParseResult::Error => panic!("Could not parse the line")
                };
            }
            file_system.print();
            file_system.get_first_challenge();
            file_system.get_second_challenge();
        }
    }else{
        println!("Error reading from file")
    }
}