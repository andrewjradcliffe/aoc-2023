/*! Zero-copy implementation, because I felt like it.
 */

use std::convert::TryFrom;
use std::{fs, io, path::Path};

pub fn hash(s: &str) -> u8 {
    s.chars()
        .fold(0u8, |h, c| h.wrapping_add(c as u8).wrapping_mul(17))
}

pub fn init_seq_sum(s: &str) -> u32 {
    s.split(',').map(|x| hash(x) as u32).sum()
}
pub fn init_seq_from_path<T: AsRef<Path>>(path: T) -> io::Result<String> {
    let mut s = fs::read_to_string(path)?;
    while s.ends_with('\n') {
        s.pop();
    }
    Ok(s)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lens<'a> {
    label: &'a str,
    focal: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashMap<'a> {
    // Whether this should be stack or heap allocated is a reasonable
    // question to ask, as this will be 2^8 * (3 * 2^3) bytes on the stack.
    // Realistically, 6KiB is too big to confer advantages.
    // boxes: [Vec<Lens<'a>>; 256],
    boxes: Vec<Vec<Lens<'a>>>,
}
impl<'a> HashMap<'a> {
    pub fn process(&mut self, op: Operation<'a>) {
        let idx = op.idx();
        match op {
            Dash { label } => {
                // If it were possible to have more than 1 occurrence,
                // `retain` would be better.
                // self.boxes[idx].retain(|lens| lens.label != label);
                // Instead, we can use the fact that there is at most 1
                // occurrence to do less work.
                let bin = &mut self.boxes[idx];
                if let Some(index) = bin.iter().position(|lens| lens.label == label) {
                    bin.remove(index);
                }
            }
            Equal { label, focal } => {
                let bin = &mut self.boxes[idx];
                if let Some(index) = bin.iter().position(|lens| lens.label == label) {
                    bin[index].focal = focal;
                } else {
                    bin.push(Lens { label, focal });
                }
            }
        }
    }
}

impl HashMap<'_> {
    pub fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .zip(1usize..)
            .map(|(bx, i)| {
                i * bx
                    .iter()
                    .zip(1usize..)
                    .map(|(lens, j)| lens.focal as usize * j)
                    .sum::<usize>()
            })
            .sum()
    }
}

// const ARRAY_REPEAT_VALUE: Vec<Lens<'_>> = Vec::new();
impl<'a> TryFrom<&'a str> for HashMap<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // let mut map = HashMap {
        //     boxes: [ARRAY_REPEAT_VALUE; 256],
        // };
        let mut boxes = Vec::with_capacity(256);
        boxes.resize(256, Vec::new());
        let mut map = HashMap { boxes };
        for op in s.split(',') {
            let op = Operation::try_from(op)?;
            map.process(op);
        }
        Ok(map)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation<'a> {
    Equal { label: &'a str, focal: u8 },
    Dash { label: &'a str },
}
use Operation::*;
impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if let Some((lhs, rhs)) = s.split_once('=') {
            let focal = rhs.parse::<u8>().map_err(|e| e.to_string())?;
            Ok(Equal { label: lhs, focal })
        } else if let Some((lhs, _)) = s.split_once('-') {
            Ok(Dash { label: lhs })
        } else {
            Err(s.to_string())
        }
    }
}

impl Operation<'_> {
    pub fn idx(&self) -> usize {
        match self {
            Equal { label, .. } => hash(label) as usize,
            Dash { label } => hash(label) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_works() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);

        let lhs: Vec<_> = TEST.split(',').map(hash).collect();
        let rhs = vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn init_seq_sum_works() {
        assert_eq!(init_seq_sum(TEST), 1320);
    }

    #[test]
    fn operation_try_from() {
        let s = "rn=1";
        let lhs = Operation::try_from(s).unwrap();
        assert_eq!(
            lhs,
            Equal {
                label: "rn",
                focal: 1
            }
        );
    }

    #[test]
    fn hashmap_try_from() {
        let lhs = HashMap::try_from(TEST).unwrap();
        assert_eq!(
            lhs.boxes[0],
            vec![
                Lens {
                    label: "rn",
                    focal: 1
                },
                Lens {
                    label: "cm",
                    focal: 2
                }
            ]
        );
        assert_eq!(
            lhs.boxes[3],
            vec![
                Lens {
                    label: "ot",
                    focal: 7
                },
                Lens {
                    label: "ab",
                    focal: 5
                },
                Lens {
                    label: "pc",
                    focal: 6
                }
            ]
        );
    }
    #[test]
    fn focusing_power() {
        let map = HashMap::try_from(TEST).unwrap();
        assert_eq!(map.focusing_power(), 145);
    }
}
