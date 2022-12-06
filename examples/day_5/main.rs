use std::fs;
use std::io::Read;

#[derive(Debug)]
enum CraneType {
    M9000,
    M9001
}
#[derive(Debug)]
struct Crane{
    stacks: Vec<Vec<char>>,
    operations: Vec<Operation>,
    crane_type: CraneType
}

impl Crane {
    pub fn apply_operations(&mut self) {
        self.operations.reverse();
        for stack in &mut self.stacks {
            stack.reverse()
        }
        match self.crane_type {
            CraneType::M9000 => {
                while let Some(operation) = self.operations.pop() {
                    for _ in 0..operation.count {
                        if let Some(cr) = self.stacks[operation.from].pop() {
                            self.stacks[operation.to].push(cr);
                        }
                    }
                }
            },
            CraneType::M9001 => {
                while let Some(operation) = self.operations.pop() {
                    let mut buffer = Vec::new();
                    for _ in 0..operation.count {
                        if let Some(cr) = self.stacks[operation.from].pop() {
                            buffer.push(cr);
                        }
                    }
                    buffer.reverse();
                    for cr in buffer {
                        self.stacks[operation.to].push(cr);
                    }
                }
            }
        }
        
    }
}

#[derive(Debug)]
struct Operation {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Operation {
    fn from(operation_str: &str) -> Self {
        // move 5 from 4 to 9
        let parts = operation_str.split(' ').collect::<Vec<&str>>();
        Operation {
            count: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_5/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {

            let mut header_read: bool = false;
            let mut crane = Crane{
                stacks: Vec::new(),
                operations: Vec::new(),
                crane_type: CraneType::M9001
            };
            for line in contents.split("\r\n") {
                if line.is_empty() {header_read = true;}
                else{
                    match header_read {
                        true => {
                            //Reading operations
                            crane.operations.push(Operation::from(line));
                        }
                        false => {
                            for i in 0..=line.len() / 4 {
                                //Get the ascii char representing the crate
                                let cr = line.chars().nth((i*4)+1).unwrap();
                                //If the stack does not exist, create it
                                if crane.stacks.len() < i+1 {crane.stacks.push(Vec::new());}
                                //If the create is actually a letter, push it to the stack
                                if !cr.is_ascii_whitespace() && !cr.is_ascii_digit() {crane.stacks[i].push(cr);}
                            }
                        }
                    }
                }
            }
            println!("Stacks: {:?}", crane.stacks);
            println!("Operations: {:?}", crane.operations);
            crane.apply_operations();
            println!("Stacks: {:?}", crane.stacks);
            println!("Operations: {:?}", crane.operations);
            for i in 0..crane.stacks.len() {
                print!("{}", crane.stacks[i].last().unwrap_or(&'\0'))
            }
            println!();


            // let mut header = Vec::new();
            // let mut operations = Vec::new();
            // let mut offset = 1usize;
            
            // for line in contents.split("\r\n") {
            //     if line.is_empty() {
            //         break;
            //     }
            //     header.push(line);
            //     offset += 1;
            // }

            // for line in contents.split("\r\n").skip(offset) {
            //     operations.push(line);
            // }

            // println!("Header: {header:?}");
            // println!("Operations: {operations:?}");
        }
    }else{
        println!("Could not open file");
    }
}