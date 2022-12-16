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

            for item in s[1..s.len()-1].split(',') {
                result.push(Self::from_str(item).unwrap())
            }
            Ok(Self::Vec(result))
        }else{
            Ok(Self::Integer(s.parse::<i32>().unwrap()))
        }
    }
}

struct Packet {
    left: Field,
    right: Field,
}


fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_13/input_short") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() { 
            let mut packets: Vec<Packet> = Vec::new();
            
            for line in  contents.split("\r\n").filter_map(|f| if !f.is_empty() { Some(f) }else{None}).collect::<Vec<&str>>().windows(2)
            {
                let left = Field::from_str(&line[0]).unwrap();
                let right = Field::from_str(&line[1]).unwrap();
                packets.push(Packet{left,right});
            }            

            let sum = packets.iter().enumerate().filter(|(idx, p)| p.left < p.right ).map(|(idx, p)| idx + 1).sum::<usize>();
            println!("Final sum {sum}");
            
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
}