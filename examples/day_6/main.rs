use std::fs;
use std::io::Read;


fn find_marker(str: &str, spacing: usize) -> usize{
 
    let string = String::from(str);  
    let window = string.chars().collect::<Vec<char>>();
    for (idx, chars) in window.windows(spacing).enumerate() {
        let mut is_repeated = false;
        'outer: for i in 0..=spacing-2 {
            for b in i+1..=spacing-1{
                if chars[i] == chars[b] {
                    is_repeated = true;
                    break 'outer;
                }
            }
        }
        if !is_repeated {
            return idx + spacing
        }

    }
    0
}

fn find_start_of_message(str: &str) -> usize{
   find_marker(str, 14)
}

fn find_start_of_packet(str: &str) -> usize{
    find_marker(str, 4)
}
fn main(){
    if let Ok(mut file) = fs::File::open("examples/day_6/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            println!("Start of packet {}", find_start_of_packet(contents.as_str()));
            println!("Start of message {}", find_start_of_message(contents.as_str()));
        }
    }else{
        println!("Error reading from file")
    }
}

#[cfg(test)]
mod test{
    use crate::{find_start_of_packet, find_start_of_message};

    #[test]
    fn test_find_start_of_packet(){
        assert_eq!(find_start_of_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(find_start_of_packet("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_of_packet("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_start_of_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_start_of_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_find_start_of_message(){
        assert_eq!(find_start_of_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_of_message("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_of_message("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_start_of_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_start_of_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}