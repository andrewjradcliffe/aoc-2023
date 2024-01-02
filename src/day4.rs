use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    id: usize,
    winning: Vec<u32>,
    have: Vec<u32>,
    win_count: usize,
}
impl Card {
    pub fn new(id: usize, winning: Vec<u32>, have: Vec<u32>) -> Self {
        let win_count = have.iter().filter(|x| winning.contains(x)).count();
        Self {
            id,
            winning,
            have,
            win_count,
        }
    }
    pub fn points(&self) -> u64 {
        let n = self.win_count as usize;
        if n == 0 {
            0
        } else {
            1 << (n - 1)
        }
    }

    pub fn count_copies(&self, cards: &[Card]) -> usize {
        (self.id..self.id + self.win_count)
            .map(|i| {
                let card = &cards[i];
                card.count_copies(cards)
            })
            .sum::<usize>()
            + 1
    }
}

#[derive(Debug)]
pub enum ParseCardError {
    Int(ParseIntError),
    Other(String),
    NoId(String),
}

macro_rules! from_err {
    {$T:path, $U:path, $V:ident} => {
        impl From<$T> for $U {
            fn from(e: $T) -> Self {
                Self::$V(e)
            }
        }
    }
}
from_err! {ParseIntError, ParseCardError, Int}

impl FromStr for Card {
    type Err = ParseCardError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once(':') {
            let mut iter = lhs.split_whitespace();
            iter.next();
            if let Some(id) = iter.next() {
                let id = id.parse::<usize>()?;
                if let Some((winning, have)) = rhs.split_once('|') {
                    let mut w = Vec::new();
                    for num in winning.trim().split_whitespace() {
                        w.push(num.parse::<u32>()?);
                    }
                    let mut h = Vec::new();
                    for num in have.trim().split_whitespace() {
                        h.push(num.parse::<u32>()?);
                    }
                    Ok(Card::new(id, w, h))
                } else {
                    Err(Self::Err::Other(s.to_string()))
                }
            } else {
                Err(Self::Err::NoId(s.to_string()))
            }
        } else {
            Err(Self::Err::Other(s.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum AcquireError {
    Card(ParseCardError),
    Io(io::Error),
}
from_err! {ParseCardError, AcquireError, Card}
from_err! {io::Error, AcquireError, Io}

pub fn cards_from_file<T: AsRef<Path>>(path: T) -> Result<Vec<Card>, AcquireError> {
    let f = File::open(path.as_ref())?;
    let mut f = BufReader::new(f);
    // 1 KiB, as usual
    let mut s = String::with_capacity(1024);
    let mut cards = Vec::new();
    while f.read_line(&mut s)? != 0 {
        cards.push(s.parse::<Card>()?);
        s.clear();
    }
    Ok(cards)
}

pub fn count(cards: &[Card]) -> usize {
    cards.iter().map(|card| card.count_copies(cards)).sum()
}

pub fn sum_points(cards: &[Card]) -> u64 {
    cards.iter().map(Card::points).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn from_str() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = s.parse::<Card>().unwrap();
        assert_eq!(card.id, 1);
        assert_eq!(card.winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.have, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert_eq!(card.win_count, 4);
    }

    #[test]
    fn points() {
        let card = Card::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );
        assert_eq!(card.points(), 8);
    }

    #[test]
    fn count_copies_works() {
        let cards: Vec<_> = TEST.lines().map(|s| s.parse::<Card>().unwrap()).collect();
        assert_eq!(count(&cards), 30);
    }
}
