use std::{fs, path::Path, str::FromStr};

/*
Notation:

`t_r` : time of race
`t_c` : time of charge
`t_m` : time of move

t_r = t_c + t_m
v = t_c = t_r - t_m
d = v * t_m = t_c * t_m = t_c * (t_r - t_c)

`d_best` : best distance of race
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Race {
    t_r: u64,
    d_best: u64,
}

impl Race {
    pub fn new(t_r: u64, d_best: u64) -> Self {
        Self { t_r, d_best }
    }
    pub fn distance(&self, t_c: u64) -> u64 {
        t_c * (self.t_r - t_c)
    }
    pub fn search_space(&self) -> impl Iterator<Item = u64> + '_ {
        (0..self.t_r + 1).map(|t_c| self.distance(t_c))
    }
    /// Brute force: Θ(n)
    pub fn ways_to_win(&self) -> usize {
        let best = self.d_best.clone();
        self.search_space().filter(move |&d| d > best).count()
    }
    /// Exploit the quadratic form to do less work.
    /// This still invokes a linear search, but will perform
    /// fewer iterations than brute force in all but the worst case.
    /// Technically, this is O(n) worst case and Ω(1) best case.
    pub fn ways_to_win_bracketing(&self) -> u64 {
        let best = self.d_best.clone();
        let lhs = (0..self.t_r + 1).find(|t_c| self.distance(*t_c) > best);

        if let Some(lhs) = lhs {
            let rhs = (0..self.t_r + 1)
                .rfind(|t_c| self.distance(*t_c) > best)
                .unwrap();
            rhs - lhs + 1
        } else {
            0
        }
    }
    /// An amusing way to treat the bracketing problem:
    /// find the roots of `t_c * t_c - t_c * t_r + d_best` (in `t_c`),
    /// but because this is integer-valued, we set up the Newton iterations
    /// to iterate to a fixed point.
    ///
    /// Since we are solving a quadratic, Newton's method would converge in 1 step
    /// if we were dealing with real-valued arithmetic, but since this is integer-valued,
    /// we might require more than 1.
    ///
    /// Once the fixed point has been reached, we iterate until the objective function
    /// (`t_c * t_r - t_c * t_c`) exceeds the distance threshold. If we were dealing
    /// with real-valued arithmetic, this step would not be necessary.
    ///
    /// N.B. this is Newton's method of tangents, not the secant method
    /// (which could also be a way to solve this).
    pub fn ways_to_win_newton(&self) -> u64 {
        #[inline]
        fn g(x: i128, t_r: i128, d_best: i128) -> i128 {
            // Wrapping is intended
            x * x - x * t_r + d_best
        }
        #[inline]
        fn dg(x: i128, t_r: i128) -> i128 {
            // Wrapping is intended
            2 * x - t_r
        }
        fn newton(x_0: i128, t_r: i128, d_best: i128) -> i128 {
            let mut x: i128 = x_0;
            loop {
                let x_new = x - g(x, t_r, d_best) / dg(x, t_r);
                if x == x_new {
                    return x;
                } else {
                    x = x_new;
                }
            }
        }
        #[inline]
        fn obj(x: i128, t_r: i128) -> i128 {
            x * t_r - x * x
        }
        // We need to explicitly protect against zero values
        if self.t_r == 0 || self.d_best == 0 {
            0
        } else {
            let t_r = self.t_r.clone() as i128;
            let d_best = self.d_best.clone() as i128;
            // lower bound
            let lb = {
                let mut lb = newton(0, t_r, d_best);
                while obj(lb, t_r) <= d_best {
                    lb += 1;
                }
                lb
            };
            // upper bound
            let ub = {
                let mut ub = newton(t_r, t_r, d_best);
                while obj(ub, t_r) <= d_best {
                    ub -= 1;
                }
                ub
            };

            (ub - lb + 1) as u64
        }
    }
}

pub fn parse_races_part1(s: &str) -> Result<Vec<Race>, String> {
    if let Some((time, distance)) = s.split_once('\n') {
        if let Some((lhs, rhs)) = time.split_once(':') {
            if lhs != "Time" {
                return Err(lhs.to_string());
            }
            let times = rhs.trim().split_whitespace();
            if let Some((lhs, rhs)) = distance.split_once(':') {
                if lhs != "Distance" {
                    return Err(lhs.to_string());
                }
                let distances = rhs.trim().split_whitespace();
                let mut races = Vec::new();
                for (t_r, d_best) in times.zip(distances) {
                    let t_r = t_r.parse::<u64>().map_err(|e| e.to_string())?;
                    let d_best = d_best.parse::<u64>().map_err(|e| e.to_string())?;
                    races.push(Race::new(t_r, d_best));
                }
                Ok(races)
            } else {
                Err(distance.to_string())
            }
        } else {
            Err(time.to_string())
        }
    } else {
        Err(s.to_string())
    }
}

pub fn races_from_path_part1<T: AsRef<Path>>(path: T) -> Result<Vec<Race>, String> {
    let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    parse_races_part1(&s)
}
const OFFSET: u32 = '0' as u32;
const BASE: u32 = 10;
impl FromStr for Race {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Common helper
        fn mixed_ascii_to_u64(s: &str) -> Result<u64, String> {
            if s.len() == 0 {
                Err("empty string is not a valid input".to_string())
            } else {
                let mut n: u64 = 0;
                for c in s.trim().chars().filter(|c| *c != ' ') {
                    let x = (c as u32).wrapping_sub(OFFSET);
                    if x >= BASE {
                        return Err(c.to_string());
                    } else {
                        n = n * 10 + x as u64;
                    }
                }
                Ok(n)
            }
        }

        if let Some((time, distance)) = s.split_once('\n') {
            if let Some((lhs, rhs)) = time.split_once(':') {
                if lhs != "Time" {
                    return Err(lhs.to_string());
                }
                let t_r = mixed_ascii_to_u64(rhs)?;
                if let Some((lhs, rhs)) = distance.split_once(':') {
                    if lhs != "Distance" {
                        return Err(lhs.to_string());
                    }
                    let d_best = mixed_ascii_to_u64(rhs)?;
                    Ok(Race::new(t_r, d_best))
                } else {
                    Err(distance.to_string())
                }
            } else {
                Err(time.to_string())
            }
        } else {
            Err(s.to_string())
        }
    }
}

impl Race {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn search_space() {
        let x = Race { t_r: 7, d_best: 9 };
        let ds: Vec<_> = x.search_space().collect();
        assert_eq!(ds, vec![0, 6, 10, 12, 12, 10, 6, 0]);
    }

    #[test]
    fn ways_to_win() {
        let x = Race { t_r: 7, d_best: 9 };
        assert_eq!(x.ways_to_win(), 4);

        let x = Race::new(15, 40);
        assert_eq!(x.ways_to_win(), 8);
        let x = Race::new(30, 200);
        assert_eq!(x.ways_to_win(), 9);

        let x = Race::new(71530, 940200);
        assert_eq!(x.ways_to_win(), 71503);
    }
    #[test]
    fn ways_to_win_bracketing() {
        let x = Race { t_r: 7, d_best: 9 };
        assert_eq!(x.ways_to_win_bracketing(), 4);

        let x = Race::new(15, 40);
        assert_eq!(x.ways_to_win_bracketing(), 8);
        let x = Race::new(30, 200);
        assert_eq!(x.ways_to_win_bracketing(), 9);

        let x = Race::new(71530, 940200);
        assert_eq!(x.ways_to_win_bracketing(), 71503);
    }

    #[test]
    fn ways_to_win_newton() {
        let x = Race { t_r: 7, d_best: 9 };
        assert_eq!(x.ways_to_win_newton(), 4);

        let x = Race::new(15, 40);
        assert_eq!(x.ways_to_win_newton(), 8);
        let x = Race::new(30, 200);
        assert_eq!(x.ways_to_win_newton(), 9);

        let x = Race::new(71530, 940200);
        assert_eq!(x.ways_to_win_newton(), 71503);
    }

    #[test]
    fn parse_races_part1_works() {
        let lhs = parse_races_part1(TEST).unwrap();
        assert_eq!(
            lhs,
            vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
        );
    }

    #[test]
    fn from_str() {
        let x = TEST.parse::<Race>().unwrap();
        assert_eq!(x, Race::new(71530, 940200));
    }
}
