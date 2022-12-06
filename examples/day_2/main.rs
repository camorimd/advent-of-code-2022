use std::{fs, io::Read};

/// `Game.0` is my own hand
/// `Game.1` is the opponent hand
#[derive(Debug)]
struct Game(Hand, Hand);
impl Game {
    pub fn score(&self) -> usize {
        match &self.0.partial_cmp(&self.1) {
            Some(std::cmp::Ordering::Equal) => Into::<usize>::into(&self.0) + 3usize,
            Some(std::cmp::Ordering::Greater) => Into::<usize>::into(&self.0) + 6usize,
            Some(std::cmp::Ordering::Less) => Into::<usize>::into(&self.0),
            None => panic!("Should not be able")
        }
    }
}

#[derive(Debug, PartialEq)]
enum Strategy {
    Loose,
    Win,
    Draw
}

impl From<char> for Strategy {
    fn from(c: char) -> Self {
        match c {
            'X' => Strategy::Loose,
            'Y' => Strategy::Draw,
            'Z' => Self::Win,
            _ => panic!("Could not find a suitable strategy from input")
        }
    }
}

#[derive(Debug, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    pub fn select_strategy_hand(&self, strategy: Strategy) -> Hand{
        match (strategy, self) {
            (Strategy::Loose, Hand::Rock) => Hand::Scissors,
            (Strategy::Loose, Hand::Paper) => Hand::Rock,
            (Strategy::Loose, Hand::Scissors) => Hand::Paper,
            (Strategy::Win, Hand::Rock) => Hand::Paper,
            (Strategy::Win, Hand::Paper) => Hand::Scissors,
            (Strategy::Win, Hand::Scissors) => Hand::Rock,
            (Strategy::Draw, h) => (*h).clone(),
        }
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => panic!("No matching char for Game")
        }
    }
}

impl From<&Hand> for usize {
    fn from(h: &Hand) -> Self {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

impl From<Hand> for usize{
    fn from(h: Hand) -> Self {
        match h {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(match (self, other) {
            (Hand::Rock, Hand::Scissors) => std::cmp::Ordering::Greater,
            (Hand::Rock, Hand::Paper) => std::cmp::Ordering::Less,
            (Hand::Rock, Hand::Rock) => std::cmp::Ordering::Equal,
            (Hand::Scissors, Hand::Scissors) => std::cmp::Ordering::Equal,
            (Hand::Scissors, Hand::Paper) => std::cmp::Ordering::Greater,
            (Hand::Scissors, Hand::Rock) => std::cmp::Ordering::Less,
            (Hand::Paper, Hand::Scissors) => std::cmp::Ordering::Less,
            (Hand::Paper, Hand::Paper) => std::cmp::Ordering::Equal,
            (Hand::Paper, Hand::Rock) => std::cmp::Ordering::Greater,
        })
    }

    fn ge(&self, other: &Self) -> bool {
        let partial_cmp = self.partial_cmp(other);
        matches!(partial_cmp, Some(std::cmp::Ordering::Greater)) 
        ||
        matches!(partial_cmp, Some(std::cmp::Ordering::Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        let partial_cmp = self.partial_cmp(other);
        matches!(partial_cmp, Some(std::cmp::Ordering::Greater)) 
    }

    fn le(&self, other: &Self) -> bool {
        let partial_cmp = self.partial_cmp(other);
        matches!(partial_cmp, Some(std::cmp::Ordering::Less)) 
        ||
        matches!(partial_cmp, Some(std::cmp::Ordering::Equal))
    }

    fn lt(&self, other: &Self) -> bool {
        let partial_cmp = self.partial_cmp(other);
        matches!(partial_cmp, Some(std::cmp::Ordering::Less)) 
    }

    
}

/// Day 2 Rock Paper Scissors
fn main() {
    if let Ok(mut file) = fs::File::open("examples/day_2/input") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let mut games = Vec::new();
            for line in contents.split("\r\n").map(|l| l.trim()) {
                let split: Vec<char> = line.chars().collect();
                let game = Game(Hand::from(split[2]), Hand::from(split[0]));
                println!("{:?} - {}", game, game.score());
                games.push(game);
             
            }
            println!("Max score for part 1: {}", games.iter().map(|f| f.score()).sum::<usize>());

            games.clear();
            for line in contents.split("\r\n").map(|l| l.trim()) {
                let split: Vec<char> = line.chars().collect();
                //Select the oponnent hand
                let opponent_hand = Hand::from(split[0]);
                let game = Game(
                    //Now, select the strategy and choose my own hand
                    opponent_hand.select_strategy_hand(Strategy::from(split[2]))
                    , opponent_hand
                );
                println!("line: {}, {:?} - {}", line, game, game.score());
                games.push(game);
             
            }
            
            println!("Max score for part 2: {}", games.iter().map(|f| f.score()).sum::<usize>());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Hand, Game, Strategy};

    #[test]
    fn test_equality(){
        assert_eq!(Hand::Scissors, Hand::Scissors);
        assert_ne!(Hand::Paper,Hand::Rock);
        assert!(Hand::Scissors > Hand::Paper);
        assert!(Hand::Paper > Hand::Rock);
        assert!(Hand::Rock > Hand::Scissors);
    }

    #[test]
    fn test_scores(){
        assert_eq!(Game(Hand::Paper, Hand::Rock).score(), 8);
        assert_eq!(Game(Hand::Rock, Hand::Paper).score(), 1);
        assert_eq!(Game(Hand::Scissors, Hand::Scissors).score(), 6);
    }


    #[test]
    fn test_strategy_conversion(){
        assert_eq!(Strategy::from('X'), Strategy::Loose);
        assert_eq!(Strategy::from('Y'), Strategy::Draw);
        assert_eq!(Strategy::from('Z'), Strategy::Win);
    }

    #[test]
    fn test_strategy() {
        assert_eq!(Game(
            Hand::Rock.select_strategy_hand(Strategy::Draw),
            Hand::Rock
        ).score(), 4);

        assert_eq!(Game(
            Hand::Paper.select_strategy_hand(Strategy::Loose),
            Hand::Paper
        ).score(), 1);

        assert_eq!(Game(
            Hand::Scissors.select_strategy_hand(Strategy::Win),
            Hand::Scissors
        ).score(), 7);
    }
}