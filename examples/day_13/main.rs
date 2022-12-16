use std::{fs, io::Read, str::FromStr};

#[derive(Debug, Clone)]
enum Field {
    Integer(i32),
    Vec(Vec<Field>),
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Vec(r0)) => Self::Vec(vec![Self::Integer(*l0)]) == Self::Vec(r0.clone()),
            (Self::Vec(l0), Self::Integer(r0)) => Self::Vec(l0.clone()) == Self::Vec(vec![Self::Integer(*r0)]),
            (Self::Vec(l0), Self::Vec(r0)) => {
                if l0.len() == r0.len() {
                    l0.iter().enumerate().all(|(idx, field)| *field == r0[idx])
                }else{
                    false
                }
            },
        }
    }
}

// 
impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(left), Self::Integer(right)) => {
                left.partial_cmp(right)
            }
            (Self::Integer(left), Self::Vec(right)) => {
                Field::Vec(vec![Self::Integer(*left)]).partial_cmp(&Self::Vec(right.to_owned()))
            }
            (Self::Vec(left), Self::Integer(right)) => {
                Field::Vec(left.to_owned()).partial_cmp(&Self::Vec(vec![Self::Integer(*right)]))
            }
            (Self::Vec(left), Self::Vec(right)) => {
                let mut left = left.iter();
                let mut right = right.iter();

                loop {
                    let left_value = left.next();
                    let right_value = right.next();
    
                    match (left_value, right_value) {
                        (Some(Field::Integer(left)), Some(Field::Integer(right))) => {
                            match left.cmp(&right) {
                                std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less), 
                                std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                                std::cmp::Ordering::Equal => {
                                    continue;
                                    // left_value = left_iter.next();
                                    // right_value = right_iter.next();
                                }
                            }
                        }
                        (Some(left), Some(right)) => {
                            match left.partial_cmp(right) {
                                Some(std::cmp::Ordering::Less) => return Some(std::cmp::Ordering::Less), 
                                Some(std::cmp::Ordering::Greater) => return Some(std::cmp::Ordering::Greater),
                                Some(std::cmp::Ordering::Equal) => {
                                    continue;
                                },
                                None => panic!("Fuck")
                            }
                        }
                        (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                        (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                        (None, None) => return Some(std::cmp::Ordering::Equal)
                    }
                }
            }

        }
    }
}

#[derive(Debug)]
struct ParseError;
impl FromStr for Field {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next() == Some('[') {
            let mut result = vec![];
            let mut chars = s[1..s.len()-1].chars();
            let mut item = String::new();
            while let Some(char) =  chars.next(){
                let mut count = 0;
                if char == '[' {
                    count += 1;
                    item.push(char);
                    while let Some(inner_char) = chars.next() {
                        if inner_char == '[' { count += 1;}
                        if inner_char == ']' { count -= 1;}
                        if inner_char == ']' && count == 0{
                            item.push(inner_char);
                            result.push(Self::from_str(&item).unwrap());
                            item.clear();
                            break;
                        }else{
                            
                            item.push(inner_char);
                        }
                    }
                }else if char == ',' {
                    if !item.is_empty() {
                        result.push(Self::from_str(&item).unwrap());
                        item.clear();
                    }
                }else{
                    item.push(char);
                }
            }
            if !item.is_empty() {
                result.push(Self::from_str(&item).unwrap());
            }
            Ok(Self::Vec(result))
        }else{
            Ok(Self::Integer(s.parse::<i32>().unwrap()))
        }
    }
}

#[derive(Debug)]
struct Packet {
    left: Field,
    right: Field,
}

impl Packet {
    fn is_correct(&self) -> bool {
        // println!("{}", self.left < self.right);
        // println!("\t{:?}", self.left);
        // println!("\t{:?}", self.right);
        self.left < self.right
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_13/input_long") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let mut packets: Vec<Packet> = Vec::new();

            let all = contents.split("\r\n").filter_map(|f| if !f.is_empty() { Some(f) }else{None}).collect::<Vec<&str>>();
            let left = all.iter().step_by(2);
            let right = all.iter().skip(1).step_by(2);


            
            for line in  left.zip(right)
            {
                let left = Field::from_str(line.0).unwrap();
                let right = Field::from_str(line.1).unwrap();
                packets.push(Packet{left,right});
            }            

            let sum = packets.iter().enumerate().filter(|(_, p)| p.is_correct()).map(|(idx, p)| idx + 1).sum::<usize>();
            println!("Final sum {sum:?}");
            let mut fields = packets.iter().map(|f| vec![f.left.clone(), f.right.clone()]).flatten().collect::<Vec<Field>>();
            let first_code = Field::Vec(vec![Field::Vec(vec![Field::Integer(2)])]);
            let second_code = Field::Vec(vec![Field::Vec(vec![Field::Integer(6)])]);
            fields.append(&mut vec![
                Field::Vec(vec![Field::Vec(vec![Field::Integer(2)])]),
                Field::Vec(vec![Field::Vec(vec![Field::Integer(6)])]),
            ]);
            fields.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for field in &fields {
                println!("{field:?}");
            }

            let mut total = 1;
            for idx in fields.iter().enumerate().filter(|(_, f)| *f.clone() == first_code || *f.clone() == second_code).map(|(idx,_)| idx + 1){
                total *= idx;
            }

            println!("{total}");

            
            
            
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field_struct_ordering(){
        assert!(Field::Integer(1) == Field::Integer(1));
        assert_eq!(
            Field::Vec(vec![Field::Integer(1), Field::Integer(2), Field::Integer(3)]),
            Field::Vec(vec![Field::Integer(1), Field::Integer(2), Field::Integer(3)])
        );
        assert_eq!(
            Field::Vec(vec![Field::Integer(1), Field::Vec(vec![Field::Integer(2)]), Field::Integer(3)]),
            Field::Vec(vec![Field::Integer(1), Field::Vec(vec![Field::Integer(2)]), Field::Integer(3)])
        );

        assert!(Field::Integer(1) < Field::Integer(2));
        assert!(Field::Integer(2) > Field::Integer(1));
        assert!(Field::Integer(2) != Field::Integer(1));

        assert!(Field::Vec(vec![Field::Integer(2), Field::Integer(3), Field::Integer(4)]) < Field::Vec(vec![Field::Integer(4)]));
        assert!(
            Field::Vec(vec![
                Field::Vec(vec![Field::Integer(1)]),
                Field::Vec(vec![Field::Integer(2),Field::Integer(3),Field::Integer(4)])
            ])
            <
            Field::Vec(vec![
                Field::Vec(vec![Field::Integer(1)]),Field::Integer(4)
            ]),
        "Left side is not smaller");


        assert!(
            Field::Vec(vec![Field::Integer(1),Field::Integer(1),Field::Integer(3),Field::Integer(1),Field::Integer(1)])
            <
            Field::Vec(vec![Field::Integer(1),Field::Integer(1),Field::Integer(5),Field::Integer(1),Field::Integer(1)])
        );

        assert!(
            Field::Vec(vec![Field::Integer(9)]) >
            Field::Vec(vec![Field::Vec(vec![Field::Integer(8),Field::Integer(7),Field::Integer(6)])])
        );

        assert!(
            Field::Vec(vec![Field::Vec(vec![Field::Integer(4),Field::Integer(4)]),Field::Integer(4),Field::Integer(4)])
            <
            Field::Vec(
                vec![
                    Field::Vec(vec![Field::Integer(4),Field::Integer(4)]),
                    Field::Integer(4),Field::Integer(4),Field::Integer(4)])
        );

    }

    #[test]
    fn test_pairs(){
        assert!(Field::from_str("[1,1,3,1,1]").unwrap() < Field::from_str("[1,1,5,1,1]").unwrap());
        assert!(Field::from_str("[[1],[2,3,4]]").unwrap() < Field::from_str("[[1],4]").unwrap());
        assert!((Field::from_str("[9]").unwrap() < Field::from_str("[[8,7,6]]").unwrap()) == false);
    }
}