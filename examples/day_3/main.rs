//Again I will do everything *MUSK* style... verbose as Hell! :P
use std::{io::Read, fs, collections::HashSet};
use array_tool::vec::*;
struct ElveGroup<'a> {
    elves: Vec<Rucksack<'a>>
}

struct Rucksack<'a> {
    full_storage: &'a str,
    first_compartment: Vec<char>,
    second_compartment: Vec<char>,
    shared_types: Vec<char>
}

impl<'a> Rucksack<'a> {
    pub fn from_line(line: &'a str) -> Self {
        let chars = line.chars().collect::<Vec<char>>();
        let first_compartment = chars[0..(chars.len() / 2)].to_vec();
        let second_compartment = chars[chars.len () / 2..].to_vec();
        let mut shared_types = Vec::new();

        for c in &first_compartment {
            if second_compartment.contains(c) {
                shared_types.push(*c);
            }
        }
        shared_types = shared_types.into_iter().collect::<HashSet<_>>().into_iter().collect();

        Self {
            full_storage: line,
            first_compartment,
            second_compartment,
            shared_types
        }
    }

    pub fn get_priority(&self) -> usize {
        let mut priority = 0usize;
        for c in &self.shared_types {
            match c {
                'a'..='z' => priority += (*c as u32 - 96u32) as usize,
                'A'..='Z' => priority += (*c as u32 - 65u32 + 27u32) as usize,
                other => panic!("NOOO {}", *other)
            }
        }

        priority
    }
}

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_3/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let mut rucksacks = Vec::new();
            let mut groups = Vec::new();
            let mut elve_group = None;
            for (idx,line) in contents.split("\r\n").map(|l|l.trim()).enumerate() {
                if idx % 3 == 0 {
                    if elve_group.is_some() {
                        groups.push(elve_group);
                    }
                    elve_group = Some(ElveGroup{elves: Vec::new()});
                }

                rucksacks.push(Rucksack::from_line(line));
                elve_group = elve_group.take().map(|mut f| {f.elves.push(Rucksack::from_line(line)); f});
            }
            groups.push(elve_group);
            println!("Total priorities {}", rucksacks.iter().map(|f| f.get_priority()).sum::<usize>());


            let groups_collection = groups.iter().map(|f| {
                let group = f.as_ref().expect("The group exists");
                group.elves[0].full_storage.chars().collect::<Vec<char>>()
                    .intersect(group.elves[1].full_storage.chars().collect::<Vec<char>>())
                .intersect(
                    group.elves[2].full_storage.chars().collect::<Vec<char>>()
                )
            }).collect::<Vec<_>>();

            let summer = groups_collection.iter().flat_map(|f| f.iter()).map(|c| {
                let result = match *c {
                    'a'..='z' => (*c as u32 - 96u32) as usize,
                    'A'..='Z' => (*c as u32 - 65u32 + 27u32) as usize,
                    other => panic!("NOOO {}", other)
                };
                println!("{} {}", c, result);
                (*c, result)
            }).collect::<Vec<(char, usize)>>();

            let total = summer.iter().fold(0usize, |accum, f| {
                println!("{} + {} ", accum, f.1);
                accum + f.1
            });

            println!("Total groups {}", groups.len());
            println!("Groups collection {:?}", groups_collection);
            println!("Total priorities {:?}", total);

            
        }
        
    }
}




#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_compartments(){
        let test  = "vJrwpWtwJgWrhcsFMMfFFhFp\r\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\r\n\
        PmmdzqPrVvPwwTWBwg\r\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\r\n\
        ttgJtRGJQctTZtZT\r\n\
        CrZsJsPPZsGzwwsLwLmpwMDw";

        let priorities = [16, 38, 42, 22, 20, 19];

        let rucksack = Rucksack::from_line("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(
            rucksack.first_compartment, 
            "vJrwpWtwJgWr".chars().collect::<Vec<char>>()
        );

        assert_eq!(
            rucksack.second_compartment,
            "hcsFMMfFFhFp".chars().collect::<Vec<char>>()
        );

        assert_eq!(*rucksack.shared_types.last().unwrap(), 'p');
        assert_eq!(rucksack.get_priority(), 16);


        for (idx, line) in test.split("\r\n").map(|f| f.trim()).enumerate() {
            let rucksack = Rucksack::from_line(line);
            assert_eq!(rucksack.get_priority(), priorities[idx]);
        }

        let mut group1 = ElveGroup{elves: Vec::new()};
        let mut group2 = ElveGroup{elves:Vec::new()};
        let lines = test.split("\r\n").collect::<Vec<&str>>();
        group1.elves.push(Rucksack::from_line(lines[0]));
        group1.elves.push(Rucksack::from_line(lines[1]));
        group1.elves.push(Rucksack::from_line(lines[2]));
        group2.elves.push(Rucksack::from_line(lines[3]));
        group2.elves.push(Rucksack::from_line(lines[4]));
        group2.elves.push(Rucksack::from_line(lines[5]));

        let group1_badge = group1.elves[0].full_storage.chars().collect::<Vec<char>>()
            .intersect(group1.elves[1].full_storage.chars().collect::<Vec<char>>())
            .intersect(
                group1.elves[2].full_storage.chars().collect::<Vec<char>>()
            );

        assert_eq!(group1_badge, vec!['r']);

        let group2_badge = group2.elves[0].full_storage.chars().collect::<Vec<char>>()
            .intersect(group2.elves[1].full_storage.chars().collect::<Vec<char>>())
            .intersect(
                group2.elves[2].full_storage.chars().collect::<Vec<char>>()
            );

        assert_eq!(group2_badge, vec!['Z']);
       
        let prio1 =  group1_badge.iter().map(|c|  
            match *c {
                    'a'..='z' => (*c as u32 - 96u32) as usize,
                    'A'..='Z' => (*c as u32 - 65u32 + 27u32) as usize,
                    other => panic!("NOOO {}", other)
                }
        ).sum::<usize>();
        let prio2 =  group2_badge.iter().map(|c|  
            match *c {
                    'a'..='z' => (*c as u32 - 96u32) as usize,
                    'A'..='Z' => (*c as u32 - 65u32 + 27u32) as usize,
                    other => panic!("NOOO {}", other)
                }
        ).sum::<usize>();

        assert_eq!(prio1, 18);
        assert_eq!(prio2, 52);
        assert_eq!(prio1 + prio2, 70);


    }
}