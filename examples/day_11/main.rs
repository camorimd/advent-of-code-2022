use std::{fs, io::Read, str::FromStr};

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
enum OperandType {
    Constant(i128),
    Old
}

#[derive(Debug)]
enum OperationType {
    Addition,
    Substraction,
    Multiplication,
    Division
}

#[derive(Debug)]
struct Operation {
    operand_a: OperandType,
    operand_b: OperandType,
    operation_type: OperationType
}

impl Operation {
    fn call(&self, old_value: i128, global_modifier: i128) -> i128{
        let operand_a = {
            match self.operand_a {
                OperandType::Constant(val) => val,
                OperandType::Old => old_value
            }
        };
        let operand_b = {
            match self.operand_b {
                OperandType::Constant(val) => val,
                OperandType::Old => old_value
            }
        };

        let r = match self.operation_type {
            OperationType::Addition => { operand_a + operand_b}
            OperationType::Division => { operand_a / operand_b}
            OperationType::Multiplication => {operand_a * operand_b}
            OperationType::Substraction => {operand_a - operand_b}
        };

        //https://www.reddit.com/r/adventofcode/comments/zihouc/comment/izrimjo/?utm_source=share&utm_medium=web2x&context=3
        //I don't fully understant modular arithmetic :(
        r % global_modifier
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i128>,
    operation: Operation,
    test: i128,
    test_result: (usize, usize),
    total_inspected_items: i128
}


impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\r\n").collect::<Vec<&str>>();

        //Starting items
        let mut items = Vec::<i128>::new();
        lines[1].split(':').collect::<Vec<&str>>()[1].split(',').for_each(
            |s| {
                items.push(s.trim().parse::<i128>().unwrap())
            }
        );

        //Operation
        let operation_str = lines[2].split(':').collect::<Vec<&str>>()[1].split_ascii_whitespace().collect::<Vec<&str>>();
        let operand_a = {
            match operation_str[2].trim() {
                "old" => OperandType::Old,
                other => OperandType::Constant(other.trim().parse::<i128>().unwrap())
            }
        };
        let operand_b = {
            match operation_str[4].trim() {
                "old" => OperandType::Old,
                other => OperandType::Constant(other.trim().parse::<i128>().unwrap())
            }
        };
        let operation_type = {
            match operation_str[3] {
                "*" => OperationType::Multiplication,
                "+" => OperationType::Addition,
                "/" => OperationType::Division,
                "-" => OperationType::Substraction,
                _ => panic!("Could not format")
            }
        };
        let operation = Operation {
            operand_a,
            operand_b,
            operation_type
        };

        let test = lines[3].split(':').collect::<Vec<&str>>()[1].split_ascii_whitespace().last().unwrap().parse::<i128>().unwrap();
        let test_result = {
            (
                lines[4].split(':').collect::<Vec<&str>>()[1].split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap(),
                lines[5].split(':').collect::<Vec<&str>>()[1].split_ascii_whitespace().last().unwrap().parse::<usize>().unwrap(),
            )
        };

        Ok(Self {
            items,
            operation,
            test,
            test_result,
            total_inspected_items: 0
        })

    }
}

impl Monkey {
    fn turn(&mut self, global_modifier: i128) -> Vec<(usize, i128)> {
        let mut v = Vec::<(usize, i128)>::new();

        for item in &self.items {
            let result = self.operation.call(*item, global_modifier); //copy on move.
            //result /= 3;
            if result % self.test == 0 {
                v.push((self.test_result.0, result))
            }else{
                v.push((self.test_result.1, result))
            }
            self.total_inspected_items += 1;
        }

        self.items.clear();
        v
    }

    fn push(&mut self, item: i128) {
        self.items.push(item);
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_11/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let mut monkeys = Vec::<Monkey>::new();
            let mut monkey_str = String::new();
            for line in contents.split("\r\n") {
                if !line.is_empty() {
                    monkey_str.push_str(line);
                    monkey_str.push_str("\r\n");
                }else{
                    monkeys.push(Monkey::from_str(&monkey_str).unwrap());
                    monkey_str.clear();
                }
            }
            monkeys.push(Monkey::from_str(&monkey_str).unwrap());
            // monkeys.iter().for_each(|m| println!("{m:?}"));

            let global_modifer = {
                let mut r = 1;
                for m in &monkeys {
                    r *= m.test;
                }
                r
            };

            for round in 0..10000 {
                for i in 0..monkeys.len() {
                    //take out a pointer to the monkey vec
                    let monkey = &mut monkeys[i];
                    let throws = monkey.turn(global_modifer);
                    
                    for throw in throws {
                        monkeys[throw.0].push(throw.1);
                    }
                }
                // println!("End of round {round}");
                // monkeys.iter().for_each(|m| print!(" {:?}", m.total_inspected_items));
                // println!();
            }


            let mut sorted = monkeys
                .iter().map(|m| m.total_inspected_items)
                .collect::<Vec<i128>>();
            sorted.sort();
            sorted.reverse();
            let monkey_business = sorted[0] * sorted[1];
            println!("Monkey business :{monkey_business}");

        }else{
            println!("Couldn't parse the input file");
        }
    }else{
        println!("Couldn't read the input file");
    }
}