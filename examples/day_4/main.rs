use std::fs;
use std::io::Read;

fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_4/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let mut count = 0;
            for line in contents.split("\r\n").map(|f| f.trim()) {
                let split = line.split(',').collect::<Vec<&str>>();
                let first_bound: Vec<usize> = split[0].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();
                let second_bound: Vec<usize> = split[1].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();


                if (first_bound[0]..=first_bound[1]).all(|i| (second_bound[0]..=second_bound[1]).contains(&i)){
                    println!("{:?} - {:?}", first_bound[0]..=first_bound[1], (second_bound[0]..=second_bound[1]));
                    count += 1;
                }else if (second_bound[0]..=second_bound[1]).all(|i| (first_bound[0]..=first_bound[1]).contains(&i)) {
                    println!("{:?} - {:?}", second_bound[0]..=second_bound[1], (first_bound[0]..=first_bound[1]));
                    count += 1;
                }else {
                    println!("{} - {:?} - {:?} not contained", line, first_bound, second_bound);
                }
            }
            println!("Total of complete overlapped pairs {}", count);

            let mut count = 0;
            for line in contents.split("\r\n").map(|f| f.trim()) {
                let split = line.split(',').collect::<Vec<&str>>();
                let first_bound: Vec<usize> = split[0].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();
                let second_bound: Vec<usize> = split[1].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();


                if (first_bound[0]..=first_bound[1]).any(|i| (second_bound[0]..=second_bound[1]).contains(&i)){
                    println!("{:?} - {:?}", first_bound[0]..=first_bound[1], (second_bound[0]..=second_bound[1]));
                    count += 1;
                }else if (second_bound[0]..=second_bound[1]).any(|i| (first_bound[0]..=first_bound[1]).contains(&i)) {
                    println!("{:?} - {:?}", second_bound[0]..=second_bound[1], (first_bound[0]..=first_bound[1]));
                    count += 1;
                }else {
                    println!("{} - {:?} - {:?} not contained", line, first_bound, second_bound);
                }
            }
            println!("Total of some overlapped pairs {}", count);
        }
    }else{
        println!("Error leyendo el fichero");
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_it(){
        let contents = "2-4,6-8\r\n\
        2-3,4-5\r\n\
        5-7,7-9\r\n\
        2-8,3-7\r\n\
        6-6,4-6\r\n\
        2-6,4-8";
        let mut count = 0;
        for line in contents.split("\r\n").map(|f| f.trim()) {
            let split = line.split(',').collect::<Vec<&str>>();
            let first_bound: Vec<usize> = split[0].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();
            let second_bound: Vec<usize> = split[1].split('-').collect::<Vec<&str>>().iter().map(|f| f.parse::<usize>().unwrap()).collect();


            if (first_bound[0]..=first_bound[1]).all(|i| (second_bound[0]..=second_bound[1]).contains(&i)){
                println!("{:?} - {:?}", first_bound[0]..=first_bound[1], (second_bound[0]..=second_bound[1]));
                count += 1;
            }else if (second_bound[0]..=second_bound[1]).all(|i| (first_bound[0]..=first_bound[1]).contains(&i)) {
                println!("{:?} - {:?}", second_bound[0]..=second_bound[1], (first_bound[0]..=first_bound[1]));
                count += 1;
            }else {
                println!("{} - {:?} - {:?} not contained", line, first_bound, second_bound);
            }
        }

        assert_eq!(count, 2);
    }
}