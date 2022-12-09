use std::{fs, io::{Read}, fmt::{Display}, str::FromStr};


#[derive(PartialEq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64
}

enum RopeMovement {
    Up,
    Down,
    Left,
    Right
}
#[derive(Debug)]
struct ParseMovementError;


impl FromStr for RopeMovement {
    type Err = ParseMovementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(RopeMovement::Up),
            "D" => Ok(RopeMovement::Down),
            "R" => Ok(RopeMovement::Right),
            "L" => Ok(RopeMovement::Left),
            _ => Err(ParseMovementError)
        }
    }
}

struct Rope {
    knots: Vec<Position>
}

impl Rope {



    fn move_rope(&mut self, movement: &RopeMovement){

        let head = &mut self.knots[0];
        match movement {
            RopeMovement::Up => {head.y = head.y.saturating_add(1)}
            RopeMovement::Down => {head.y = head.y.saturating_sub(1)}
            RopeMovement::Right => {head.x = head.x.saturating_add(1)}
            RopeMovement::Left => {head.x = head.x.saturating_sub(1)}
        }

        for i in 0..self.knots.len()-1{
            let head = self.knots[i];
            if !self.is_adjacent(head, self.knots[i+1]) {
                {
                    self.move_tail(i, i + 1);
                }               
            }
        }
        
    }

    fn move_tail(&mut self, head_idx: usize, tail_idx: usize) {
        let head = self.knots[head_idx]; // Creates a copy
        let tail = &mut self.knots[tail_idx];
        
        match tail.y.cmp(&head.y) {
            std::cmp::Ordering::Greater => tail.y = tail.y.saturating_sub(1),
            std::cmp::Ordering::Less => tail.y = tail.y.saturating_add(1),
            _ => {}
        }

        match tail.x.cmp(&head.x) {
            std::cmp::Ordering::Less => tail.x = tail.x.saturating_add(1),
            std::cmp::Ordering::Greater => tail.x = tail.x.saturating_sub(1),
            _ => {}
        }
    }

    fn is_adjacent(&self, head: Position, tail: Position) -> bool {
        let mut adjacent_positions = Vec::new();
        for x in head.x.saturating_sub(1)..=(head.x+1) {
            for y in head.y.saturating_sub(1)..=(head.y+1){
                adjacent_positions.push(Position{x, y});
            }
        }
        adjacent_positions.contains(&tail)
    }

    fn new(knots: usize) -> Self {
        let knots = vec![Position{x:0,y:0};knots+1];       
        Self{
            knots
        }
    }


    fn tail(&self) -> &Position {
        &self.knots[self.knots.len()-1]
    }

    fn head(&self) -> &Position {
        &self.knots[0]
    }

    fn position_has_knot(&self, position: &Position) -> Option<usize> {
        self.knots.iter().position(|f| *f == *position)
    }

}

struct State {
    max_columns: usize,
    max_rows: usize,
    rope: Rope,
    visited: Vec<Position>
}

impl State {

    fn is_position_visited(&self, position: &Position) -> bool {
        self.visited.contains(position)
    }
    fn position_has_head(&self, position: &Position) -> bool {
        self.rope.head() == position
    }
    fn position_has_tail(&self, position: &Position) -> bool {
        self.rope.tail() == position
    }

    fn move_rope(&mut self, movement: &RopeMovement){
        self.rope.move_rope(movement);
        if !self.visited.contains(self.rope.tail()) {
            self.visited.push(*self.rope.tail()); // creates a copy on move
        }
    }

    fn position_has_knot(&self, position: &Position) -> Option<usize> {
        self.rope.position_has_knot(position)
    }

}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0i64..self.max_rows as i64).rev() {
            for x in 0i64..self.max_columns as i64 {
                let current = Position {x, y};
                if self.position_has_head(&current) {
                    write!(f, "H")?;
                }else if self.position_has_tail(&current) {
                    write!(f, "T")?;
                }else if let Some(idx) = self.position_has_knot(&current){
                    write!(f, "{idx}")?;
                }else if self.is_position_visited(&current) {
                    write!(f, "#")?;
                }else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main(){

    if let Ok(mut file) = fs::File::open("examples/day_9/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            //Let's assume the grid is square, we can take the MAX of the movements
            let max = contents.split("\r\n")
                .map(|f| f.split_whitespace())
                .map(|f| f.collect::<Vec<&str>>())
                .map(|f| f[1].parse::<usize>().unwrap_or(0usize))
                .max();

            if let Some(max) = max {
                let mut state = State {
                    max_columns: max+1,
                    max_rows: max+1,
                    rope: Rope::new(9),
                    visited: Vec::new()
                };

                println!("{state}");
                for movement in contents.split("\r\n").enumerate() {
                    let instruction = movement.1.split_ascii_whitespace().collect::<Vec<&str>>();
                    let movement = RopeMovement::from_str(instruction[0]).unwrap();
                    let count = instruction[1].parse::<usize>().unwrap();
                    for _ in 0..count {
                        state.move_rope(&movement);
                        // std::io::stdin().read_line(&mut String::new()).unwrap();
                    }
                }
                println!("{state}");
                
                println!("\nThere were this many visited: {}", state.visited.len());

            }else{
                println!("Couldn't parse max grid size");
            }
                
        }else{
            println!("Couldn't parse the input file");
        }
    }else{
        println!("Couldn't read the input file");
    }

}