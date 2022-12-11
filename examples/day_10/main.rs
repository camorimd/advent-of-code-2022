use std::{fs, io::Read, str::FromStr, ops::Range};

#[derive(Debug)]
struct Cpu {
    x: i64,
    cycle: u64,
    current_instruction: Option<CpuInstruction>,
    ram: Vec<String>,
    crt_position: (u64, u64)
}

#[derive(Debug, Clone)]
enum InstructionType {
    Noop,
    Add(i32)
}

#[derive(Clone, Debug)]
struct CpuInstruction {
    instruction_type: InstructionType,
    cycles_left: i32
}

impl CpuInstruction {
    fn new(instruction_type: InstructionType) -> Self {
        let cycles_left = instruction_type.cycles();
        Self {
            instruction_type,
            cycles_left
        }
    }
}

impl FromStr for CpuInstruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction_type = InstructionType::from_str(s)?;
        Ok(CpuInstruction::new(instruction_type))
    }
}

impl InstructionType {
    fn cycles (&self) -> i32 {
        match self {
            InstructionType::Noop => 1,
            InstructionType::Add(_) => 2
        }
    }
}

#[derive(Debug)]
struct ParseError;
impl FromStr for InstructionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = s.split_whitespace().collect::<Vec<&str>>();
        match sep[0] {
            "noop" => Ok(InstructionType::Noop),
            "addx" => Ok(InstructionType::Add(sep[1].parse::<i32>().unwrap())),
            _ => Err(ParseError)
        }
    }
}

impl Cpu {
    fn new(commands: String) -> Self {
        let ram = commands.split("\r\n").map(String::from).collect();
        Self { x: 1, cycle: 0, current_instruction: None, ram, crt_position: (1,0)}
    }

    fn execute(&mut self){

        //Let's move a single position on the screen that is 40 "pixels" wide
        let x = self.cycle % 40;
        let y = self.cycle / 40;

        if self.crt_position.1 < y {
            println!()
        }
        
        self.crt_position = (x, y);
        //Determine the "sprite" position 
        let sprite = vec![self.x-1, self.x, self.x+1];
        if sprite.contains(&(self.crt_position.0 as i64)) {
            print!("#");
        }else{
            print!(".");
        }


        if let Some(mut instruction) = self.current_instruction.take() {
            self.current_instruction = {
                match instruction.instruction_type {
                    InstructionType::Add(val) => {
                        if instruction.cycles_left == 1 {
                            self.x += val as i64;
                            None
                        }else{
                            instruction.cycles_left -= 1;
                            Some(instruction)
                        }
                    }
                    InstructionType::Noop => {
                        None
                    }
                }
            }
        }

        self.cycle += 1;        
    }
    
    fn fetch(&mut self, command: String){
        let instruction = CpuInstruction::from_str(command.as_str());
        if let Ok(instruction) = instruction {
            self.current_instruction = Some(instruction)
        }
    }

    fn do_cycle(&mut self) {
        let range = Range {start: 20, end: 10000}.step_by(40).collect::<Vec<u64>>();
        let mut signal_strength = Vec::new();
        for command in self.ram.clone() {
            if self.current_instruction.is_none() {
                self.fetch(command)
            }
            while self.current_instruction.is_some() {
                self.execute();

                if range.contains(&self.cycle){
                    // println!("({}) Current Instruction {:?} , Register {}", self.cycle, self.current_instruction, self.x);
                    signal_strength.push(self.x * self.cycle as i64);
                }
            }
        }
        println!("\nExecution stopped signal_strength {}", signal_strength.iter().sum::<i64>())
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_10/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let mut cpu = Cpu::new(contents);
            cpu.do_cycle();
        }else{
            println!("Couldn't parse the input file");
        }
    }else{
        println!("Couldn't read the input file");
    }
}