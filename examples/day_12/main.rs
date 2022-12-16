use std::{fs, io::{Read, Write}, collections::{HashMap}, cmp::Reverse, thread::JoinHandle};

use crossterm::{QueueableCommand, cursor, style::{self}, terminal};
use priority_queue::PriorityQueue;
use tokio::task;

#[derive(Debug, Copy, Clone)]
struct Node {
    x: u64,
    y: u64,
    height: u64
}

#[derive(Debug, Clone)]
struct HeightMap  {
    height: u64,
    width: u64,
    inner_vec: Vec<Node>
}

impl HeightMap {
    fn new(str: String) -> Self {
        let height = str.split("\r\n").count() as u64;
        let width = str.split("\r\n").collect::<Vec<&str>>()[0].chars().count() as u64;
        let inner_vec = str.split("\r\n")
            .collect::<Vec<&str>>()
            .concat().chars().enumerate().into_iter().map(|(idx, char)|{
                let y = idx as u64 / width;
                let x = idx as u64 % width;
                let height = char as u64;

                Node {
                    x,
                    y,
                    height
                }
            })
            .collect();

        Self {
            height,
            width,
            inner_vec
        }
    }

    fn start_node_idx(&self) -> usize {
        self.inner_vec.iter().position(|f| f.height == 'S' as u64).unwrap()
    }

    fn end_node_idx(&self) -> usize {
        self.inner_vec.iter().position(|f| f.height == 'E' as u64).unwrap()
    }

    fn get_node(&self, idx: usize) -> &Node {
        &self.inner_vec.get(idx).unwrap()
    }

    fn get_node_mut(&mut self, idx: usize) -> &mut Node {
        self.inner_vec.get_mut(idx).unwrap()
    }

    fn get_neighours(&self, idx: usize) -> Vec<&Node> {
        let y = idx as u64 / self.width;
        let x = idx as u64 % self.width;
        let mut result = vec![];

        let current = self.get_node(idx);

        for n in vec![
            (Some(x), y.checked_add(1)),
            (Some(x), y.checked_sub(1)),
            (x.checked_add(1), Some(y)),
            (x.checked_sub(1), Some(y))
        ] {
            let n_x = match n.0 {
                Some(x) => x,
                None => continue
            };
            let n_y = match n.1 {
                Some(y) => y,
                None => continue
            };

            if n_x < self.width && n_y < self.height {
                let n_idx = n_x + (n_y * self.width);
                let candidate = self.inner_vec.get(n_idx as usize).unwrap();

                let mut current_height = current.height;
                let mut candidate_height = candidate.height;

                if current_height as u8 as char == 'E' { current_height = 'z' as u64}
                if current_height as u8 as char == 'S' { current_height = 'a' as u64}
                if candidate_height as u8 as char == 'E' { candidate_height = 'z' as u64}
                if candidate_height as u8 as char == 'S' { candidate_height = 'a' as u64}
                
                
                if candidate_height.abs_diff(current_height) <= 1 
                || candidate_height < current_height
                {
                    result.push(candidate);
                }
            }
        }        
        result
    }

    fn get_node_idx(&self, node: &Node) -> usize{
        (node.x + node.y * self.width) as usize
    }

    fn distance(&self, start_idx: usize, end_idx: usize) -> i32{
        let start_node = self.get_node(start_idx);
        let end_node = self.get_node(end_idx);
        (start_node.x.abs_diff(end_node.x) + start_node.y.abs_diff(end_node.y)) as i32
    }

    fn get_height(&self, char: char) -> Vec<usize>{
        self.inner_vec
            .iter()
                .filter(|f| f.height as u8 as char == char)
                .map(|c| self.get_node_idx(c))
            .collect()
    }

}

#[tokio::main]
async fn main(){
    // let mut stdout = std::io::stdout();
    if let Ok(mut file) = fs::File::open("examples/day_12/input_long") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let map = HeightMap::new(contents);
            let mut starting_points = map.get_height('a');
            starting_points.push(map.start_node_idx());

            let mut handles = vec![];

            for starting_point in starting_points {
                let map = map.clone();
                handles.push(
                    task::spawn(async move {
                        // let manhattan            
                        let mut frontier: PriorityQueue<usize, Reverse<i32>> = PriorityQueue::new();
                        let mut came_from = HashMap::new();
                        let mut cost_so_far = HashMap::new();

                        //Wel... This will take a looong way
                        frontier.push(starting_point, Reverse(0));
                        came_from.insert(starting_point, None);
                        cost_so_far.insert(starting_point, 0);
                        
                        while let Some((current, _)) = frontier.pop() {

                            if current == map.end_node_idx() {
                                break;
                            }

                            for child in map.get_neighours(current) {
                                let child_idx = map.get_node_idx(child);

                                let new_cost = cost_so_far[&current] + 1; //

                                if !cost_so_far.contains_key(&child_idx) || new_cost < cost_so_far[&child_idx] {
                                    cost_so_far.insert(child_idx, new_cost);
                                    let priority = new_cost + map.distance(child_idx, map.end_node_idx());
                                    frontier.push(child_idx, Reverse(priority));
                                    came_from.insert(child_idx, Some(current));
                                }
                            }
                        }

                        let mut path = vec![];
                        let mut current = map.end_node_idx();
                        let mut has_path = true;
                        while current != starting_point {
                            path.push(current);
                            if came_from.get(&current).is_some() {
                                came_from.get(&current).unwrap().unwrap();
                            }else{
                                has_path = false;
                                break;
                            }
                            current = came_from.get(&current).unwrap().unwrap();
                        }
                        path.push(starting_point);

                        if has_path {
                            // stdout.queue(terminal::Clear(terminal::ClearType::Purge)).unwrap();
                            // for y in 0..map.height {
                            //     for x in 0..map.width {
                            //         stdout.queue(cursor::MoveTo(x as u16,y as u16)).unwrap();
                            //         let idx = (x + y * map.width) as usize;
                            //         if idx == starting_point {
                            //             stdout.queue(style::Print("S")).unwrap();
                            //         }else if idx == map.end_node_idx() {
                            //             stdout.queue(style::Print("E")).unwrap();
                            //         }else if path.contains(&idx) {
                            //             stdout.queue(style::Print("X")).unwrap();
                            //         }else{
                            //             stdout.queue(style::Print(".")).unwrap();
                            //         }
                            //     }
                            // }
                            // stdout.flush().unwrap();            
                            
                            path.reverse();
                            // println!("{:?}", path.len()-1);
                            Some(path.len() - 1)
                            // std::io::stdin().read_line(&mut String::new()).unwrap();
                        }else{
                            None
                        }
                    })
                );
            }

            let mut steps = vec![];
            for handle in handles {
                steps.push(handle.await.unwrap());
            }
            println!("{:?}", steps.iter().filter(|f| f.is_some()).map(|f| f.unwrap()).rfold(usize::MAX, |acc, f| if f < acc {f}else{acc} ));
        }
    }
}