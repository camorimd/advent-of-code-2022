use std::fs;
use std::io::Read;

#[derive(Debug)]
struct Tree {
    height: usize,
}
#[derive(Debug)]
struct Forest {
    rows: usize,
    columns: usize,
    trees: Vec<Tree>
}

impl Forest {

    pub fn scenic_score(&self) -> usize {
        let get_tree = |x:usize, y:usize| -> &Tree {&self.trees[y * self.columns + x]};
        let mut scenic_scores = Vec::new();

        for y in 0..self.rows {
            for x in 0..self.columns {
                let current_tree = get_tree(x, y);
                let scenic_up = {
                    let mut count = 0usize;
                    if y > 0 {
                        for other_y in (0..=y-1).rev(){
                            let t = get_tree(x, other_y);
                            count += 1;
                            if t.height >= current_tree.height { break; }
                        }
                    }
                    count
                };

                let scenic_down = {
                    let mut count = 0usize;
                    for other_y in y+1..self.rows {
                        let t = get_tree(x, other_y);
                        count += 1;
                        if t.height >= current_tree.height { break; }
                    }
                    count
                };

                let scenic_left = {
                    let mut count = 0usize;
                    if x > 0 {
                        for other_x in (0..=x-1).rev() {
                            let t = get_tree(other_x, y);
                            count += 1;
                            if t.height >= current_tree.height {break;}
                        }
                    }
                    count
                };

                let scenic_right = {
                    let mut count = 0usize;
                    for other_x in x+1..self.columns {
                        let t = get_tree(other_x, y);
                        count += 1;
                        if t.height >= current_tree.height { break; }
                    }
                    count
                };

                scenic_scores.push(scenic_up * scenic_down * scenic_left * scenic_right);
            }
        }
        scenic_scores.into_iter().max().unwrap()
    }

    pub fn count_visible(&self) -> usize {
        let get_tree = |x:usize, y:usize| -> &Tree {&self.trees[y * self.columns + x]};

        let outer = (self.rows * 2) + ((self.columns - 2) * 2);
        
        let inner = {
            let mut count = 0;
            for y in 1..self.rows - 1 {
                for x in 1..self.columns - 1 {
                    let tree = &self.trees[x + (y*self.columns)];

                    //Path to the upper edge
                    let up = {
                        let mut vec = Vec::new();
                        for other_y in 0..=y-1{
                            vec.push(get_tree(x, other_y));
                        }
                        vec
                    };
                    let left = {
                        let mut vec = Vec::new();
                        for other_x in 0..=x-1 {
                            vec.push(get_tree(other_x, y));
                        }
                        vec
                    };

                    let right = {
                        let mut vec = Vec::new();
                        
                        for other_x in x+1..self.columns {
                            vec.push(get_tree(other_x, y));
                        }
                        vec
                    };

                    let down = {
                        let mut vec = Vec::new();
                        for other_y in y+1..self.rows {
                            vec.push(get_tree(x, other_y));
                        }
                        vec
                    };

                    if  up.iter().all(|t| tree.height > t.height) || 
                        left.iter().all(|t| tree.height > t.height) ||
                        down.iter().all(|t| tree.height > t.height) ||
                        right.iter().all(|t| tree.height > t.height) 
                    { 
                        count += 1
                    }

                }
            }
            count
        };

        outer + inner
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_8/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let columns = contents.split("\r\n").count();
            let rows = contents.split("\r\n").next().unwrap().chars().count();
            let mut forest = Forest{
                columns,
                rows,
                trees: Vec::new()
            };

            for row in contents.split("\r\n") {
                for t in row.chars() {
                    forest.trees.push(
                        Tree{
                            height: t.to_digit(10).unwrap() as usize
                        }
                    )
                }
            }

            // println!("The forest {forest:?}");
            println!("There are those many visibles: {}",  forest.count_visible());
            println!("This is the perfect spot: {}",  forest.scenic_score());
        }else{
            println!("Error parsing the input file")
        }
    }else{
        println!("Couldn't read the input file")
    }
}