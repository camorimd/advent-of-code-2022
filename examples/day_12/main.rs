use std::{fs, io::{Read, Write}, collections::{HashMap}, cmp::Reverse};

use crossterm::{QueueableCommand, cursor, style::{self}, terminal};
use priority_queue::PriorityQueue;

#[derive(Debug)]
struct Node {
    x: u64,
    y: u64,
    height: u64,
    f: i64,
    g: i64,
    h: i64
}

#[derive(Debug)]
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
                let f = 0;
                let g = 0;
                let h = 0;

                Node {
                    x,
                    y,
                    height,
                    f,
                    g,
                    h
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
                if candidate_height as u8 as char == 'S' { candidate_height = 'a' as u64}
                if candidate_height as u8 as char == 'S' { candidate_height = 'a' as u64}
                
                
                if candidate_height.abs_diff(current_height) <= 1 {
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

}


fn main() {
    let mut stdout = std::io::stdout();
    if let Ok(mut file) = fs::File::open("examples/day_12/input_long") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let map = HeightMap::new(contents);

            // let manhattan

            let mut frontier: PriorityQueue<usize, Reverse<i32>> = PriorityQueue::new();
            let mut came_from = HashMap::new();
            let mut cost_so_far = HashMap::new();
            


            frontier.push(map.start_node_idx(), Reverse(0));
            came_from.insert(map.start_node_idx(), None);
            cost_so_far.insert(map.start_node_idx(), 0);
            
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


            // let vec_reached = came_from.iter().collect::<Vec<(&usize, &Option<usize>)>>();
            // vec_reached.sort();
            // println!("{vec_reached:?}");
            // println!("{}", vec_reached.len());

            let mut path = vec![];
            let mut current = map.end_node_idx();
            while current != map.start_node_idx() {
                path.push(current);
                if came_from.get(&current).is_some() {
                    came_from.get(&current).unwrap().unwrap();
                }else{
                    // panic!("{current} not found in came_from but seems to be a parent");
                    break;
                }
                current = came_from.get(&current).unwrap().unwrap();
            }
            path.push(map.start_node_idx());

            stdout.queue(terminal::Clear(terminal::ClearType::Purge)).unwrap();
            for y in 0..map.height {
                for x in 0..map.width {
                    stdout.queue(cursor::MoveTo(x as u16,y as u16)).unwrap();
                    let idx = (x + y * map.width) as usize;
                    if idx == map.start_node_idx() {
                        stdout.queue(style::Print("S")).unwrap();
                    }else if idx == map.end_node_idx() {
                        stdout.queue(style::Print("E")).unwrap();
                    }else if came_from.get(&idx).is_none() {
                        // stdout.queue(style::Print(cost_so_far[&idx])).unwrap();
                        stdout.queue(style::Print("X")).unwrap();
                    }else{
                        stdout.queue(style::Print(".")).unwrap();
                    }
                }
            }
            stdout.flush().unwrap();            
            
            path.reverse();
            println!("\n{:?}", path.len());
        }
    }
}