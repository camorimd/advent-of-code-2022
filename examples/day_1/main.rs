use std::{fs, io::Read};

type Food = u64;
struct Elve {
    snacks: Vec<Food>
}
impl Elve {
    pub fn new() -> Self {
        Self {
            snacks: Vec::new()
        }
    }
}

/// We should read the INPUT file and find the Elve with the most calories
/// We are not aiming for the shortest solution
/// We will do this **MUSK** style... Verbose as hell
fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_1/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            //Functional programming ... NOW

            let mut elves: Vec<Elve> = Vec::new();
            let mut elve = Elve::new();
            for line in contents.split("\r\n") {
                if line.is_empty() {
                    elves.push(elve);
                    elve = Elve::new();
                }else {
                    match line.trim().parse::<u64>() {
                        Ok(calories) => elve.snacks.push(calories),
                        Err(e) => println!("Error parsing: {e}")
                    }
                }
            }
            //push the last elve
            elves.push(elve);

            //Sum the elves calories and show the one with most calories
            println!("Max calories: {:?}", elves
                .iter().map(|elve| elve.snacks.iter().sum::<u64>())
                .max());


            let mut food = elves.iter().map(|elve| elve.snacks.iter().sum::<u64>())
                .collect::<Vec<Food>>();
            food.sort_by(|a, b| b.partial_cmp(a).unwrap());
            println!("Sum first 3 max calories {}", food.iter().take(3).sum::<u64>());

        }else{
            println!("Error reading file to string");
        }


    }else{
        println!("Could not open file");
    }
    
}