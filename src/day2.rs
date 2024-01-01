use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl Game {
    fn is_possible(&self, red: u8, green: u8, blue: u8) -> bool {
        let (r, g, b) = self.maximum_cubes();
        r <= red && g <= green && b <= blue
    }

    fn maximum_cubes(&self) -> (u8, u8, u8) {
        self.draws.iter().fold((0, 0, 0), |(r, g, b), Draw{red, green, blue}| {
            (r.max(*red), g.max(*green), b.max(*blue))
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Draw {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone,  PartialEq)]
pub enum ParseError {
    Number(ParseIntError),
    Color(String),
    Game(String),
}
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::Number(e)
    }
}

impl FromStr for Draw {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut x = Draw{red: 0, green: 0, blue: 0};
        for token in s.split(',') {
            if let Some((num, color)) = token.trim().split_once(' ') {
                let num = num.parse::<u8>()?;
                match color {
                    "red" => {
                        x.red = num;
                    },
                    "green" => {
                        x.green = num;
                    },
                    "blue" => {
                        x.blue = num;
                    },
                    _ => return Err(Self::Err::Color(color.to_string())),
                }
            }
        }
        Ok(x)
    }
}

impl FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Some((game, draws)) = s.split_once(':') {
            if let Some((_, id)) = game.split_once(' ') {
                let id = id.parse::<u32>()?;
                let mut x = Game{id, draws: Vec::new()};
                for draw in draws.split(';') {
                    x.draws.push(draw.parse::<Draw>()?);
                }
                Ok(x)
            } else {
                Err(Self::Err::Game(s.to_string()))
            }
        } else {
            Err(Self::Err::Game(s.to_string()))
        }
    }
}

#[derive(Debug)]
pub enum SumError {
    Parse(ParseError),
    Io(io::Error),
}
impl From<ParseError> for SumError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}
impl From<io::Error> for SumError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}



pub fn sum_possible(games: &[Game], red: u8, green: u8, blue: u8) -> u32 {
    games.into_iter().filter(move |game| game.is_possible(red, green, blue)).map(|game| {
        game.id
    }).sum()
}

pub fn sum_powerset(games: &[Game]) -> u32 {
    games.into_iter().map(|game| {
        let (r, g, b) = game.maximum_cubes();
        r as u32 * g as u32 * b as u32
    }).sum()
}

/*
The number of possible game outcomes accepted by AoC is not correct, at least,
in the sense that the "power" is not really the powerset since the empty set
(i.e. 0 cubes) is an acceptable choice for each color.

If the minimum number of cubes (required to generate the game outcomes) for each
type (color) is `n_i`, and it is permissible to choose on an arbitrary number of cubes
per type, then this is equivalent to having `n_i + 1` permissible states for each type.
Hence, the total possible outcomes for a given game setup is `∏ n_i + 1`
(where the product runs over `i`).

N.B. the prompt does not actually say that the value computed is the number of possible
game outcomes, but it would likely be more informative to change the problem
to require the logic above.
*/
pub fn sum_powerset_incl_null_set(games: &[Game]) -> u32 {
    games.into_iter().map(|game| {
        let (r, g, b) = game.maximum_cubes();
        (r + 1) as u32 * (g + 1) as u32 * (b + 1) as u32
    }).sum()
}


pub fn games_from_file<T: AsRef<Path>>(path: T) -> Result<Vec<Game>, SumError> {
    let f = File::open(path.as_ref())?;
    let mut f = BufReader::new(f);
    // 1 KiB buffer should be more than sufficient to avoid reallocations
    let mut s = String::with_capacity(1024);
    let mut games = Vec::new();
    while f.read_line(&mut s)? != 0 {
        let game = s.parse::<Game>()?;
        games.push(game);
        s.clear();
    }
    Ok(games)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_from_str() {
        let s = " 8 green, 6 blue, 20 red";
        assert_eq!(s.parse::<Draw>(), Ok(Draw{red: 20, green: 8, blue: 6}));
    }

    #[test]
    fn game_from_str() {
        let s = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let rhs = Game{id: 1, draws: vec![Draw{blue: 3, red: 4, green: 0},
                                          Draw{blue: 6, red: 1, green: 2},
                                          Draw{blue: 0, red: 0, green: 2},

        ]};
        assert_eq!(s.parse::<Game>(), Ok(rhs));


        let s = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";

        let rhs = Game{id: 3, draws: vec![Draw{blue: 6, red: 20, green: 8},
                                          Draw{blue: 5, red: 4, green: 13},
                                          Draw{blue: 0, red: 1, green: 5},

        ]};
        assert_eq!(s.parse::<Game>(), Ok(rhs));
    }
}
