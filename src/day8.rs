use std::convert::TryFrom;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    L,
    R,
}

impl TryFrom<char> for Instruction {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use Instruction::*;
        match c {
            'L' => Ok(L),
            'R' => Ok(R),
            _ => Err(c.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstructionSeq(Vec<Instruction>);

impl FromStr for InstructionSeq {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::with_capacity(s.len());
        for c in s.chars() {
            v.push(Instruction::try_from(c)?);
        }
        Ok(Self(v))
    }
}
impl From<Vec<Instruction>> for InstructionSeq {
    fn from(insns: Vec<Instruction>) -> Self {
        Self(insns)
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// pub struct Node([char; 3]);

// impl Node {
//     pub fn ends_with_a(&self) -> bool {
//         self.0[2] == 'A'
//     }
//     pub fn ends_with_z(&self) -> bool {
//         self.0[2] == 'Z'
//     }
// }

// impl From<[char; 3]> for Node {
//     fn from(id: [char; 3]) -> Self {
//         Self(id)
//     }
// }

impl FromStr for Node {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 3 {
            let mut inner = ['A'; 3];
            let mut n: u8 = 0;
            for (i, c) in s.char_indices() {
                match c {
                    'A'..='Z' => {
                        inner[i] = c;
                        n += 1;
                    }
                    _ => return Err(c.to_string()),
                }
            }
            if n == 3 {
                Ok(Self::from(inner))
            } else {
                Err(s.to_string())
            }
        } else {
            Err(s.to_string())
        }
    }
}

/// `Node` specialized for this problem.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node {
    idx: u16,
    ends_with_a: bool,
    ends_with_z: bool,
}
const OFFSET: u32 = 'A' as u32;
impl From<[char; 3]> for Node {
    fn from(id: [char; 3]) -> Self {
        let left = id[0] as u32 - OFFSET;
        assert!(left < OFFSET);
        let mid = id[1] as u32 - OFFSET;
        assert!(mid < OFFSET);
        let right = id[2] as u32 - OFFSET;
        assert!(right < OFFSET);
        // this encoding is as a linear index into a 3-dimensional array
        // of size (26, 26, 26) with 0-based indexing.
        // let idx = (left + 26 * mid + 26 * 26 * right) as u16;
        // in keeping with a positional number system, this is likely more
        // clear if we write it as (equivalently):
        let idx = (left * 26 * 26 + mid * 26 + right) as u16;
        let ends_with_a = id[2] == 'A';
        let ends_with_z = id[2] == 'Z';
        Self {
            idx,
            ends_with_a,
            ends_with_z,
        }
    }
}
impl Node {
    pub fn ends_with_a(&self) -> bool {
        self.ends_with_a
    }
    pub fn ends_with_z(&self) -> bool {
        self.ends_with_z
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tree {
    id: Node,
    left: Node,
    right: Node,
}
impl Tree {
    pub fn new(id: Node, left: Node, right: Node) -> Self {
        Self { id, left, right }
    }
}

impl FromStr for Tree {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((lhs, rhs)) = s.split_once('=') {
            let id = lhs.trim().parse::<Node>()?;
            if let Some((left, right)) = rhs.split_once(',') {
                let left = left.trim().trim_start_matches('(').parse::<Node>()?;
                let right = right.trim().trim_end_matches(')').parse::<Node>()?;
                Ok(Self { id, left, right })
            } else {
                Err(rhs.to_string())
            }
        } else {
            Err(s.to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Network {
    trees: Vec<Tree>,
}

impl From<Vec<Tree>> for Network {
    fn from(trees: Vec<Tree>) -> Self {
        // trees.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        // trees.sort_unstable_by(|a, b| a.id.idx.cmp(&b.id.idx));
        let mut t = Vec::with_capacity(17576);
        let d = Node {
            idx: 0,
            ends_with_a: false,
            ends_with_z: false,
        };
        t.resize(17576, Tree::new(d.clone(), d.clone(), d));
        for x in trees {
            let i = x.id.idx.clone() as usize;
            t[i] = x;
        }
        Self { trees: t }
    }
}

impl FromStr for Network {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = Vec::new();
        for line in s.lines() {
            trees.push(line.parse::<Tree>()?);
        }
        Ok(Self::from(trees))
    }
}

impl Network {
    // /// Complexity: O(lgn) best case, O(n) worst case.
    // pub fn push_tree(&mut self, tree: Tree) {
    //     match self.trees.binary_search_by(|x| x.id.cmp(&tree.id)) {
    //         Ok(i) => {
    //             self.trees[i] = tree;
    //         }
    //         Err(i) => {
    //             self.trees.insert(i, tree);
    //         }
    //     }
    // }
    pub fn branch(&self, insn: Instruction, node: &Node) -> Option<&Node> {
        use Instruction::*;
        // let i = self.trees.binary_search_by(|x| x.id.cmp(node)).ok()?;
        // let i = self
        //     .trees
        //     .binary_search_by(|x| x.id.idx.cmp(&node.idx))
        //     .ok()?;
        // let tree = &self.trees[i];
        let tree = &self.trees[node.idx as usize];
        match insn {
            L => Some(&tree.left),
            R => Some(&tree.right),
        }
    }
    /// Interpretation of `Result<usize, usize>`:
    /// - Ok(n)  : n > 0; `entry` terminates at `exit` after `n` branches
    /// - Err(0) : cannot traverse with empty `seq`
    /// - Err(n) : n > 0; `entry` does not terminate at `exit` after `n` branches
    pub fn traverse(&self, seq: InstructionSeq, entry: Node, exit: Node) -> Result<usize, usize> {
        let seq = seq.0;
        if !seq.is_empty() {
            let mut seq = seq.into_iter().cycle();

            let mut node = &entry;
            let mut n: usize = 0;
            while let Some(next) = self.branch(seq.next().unwrap(), node) {
                n += 1;
                if *next == exit {
                    return Ok(n);
                } else {
                    node = next;
                }
            }
            Err(n)
        } else {
            Err(0)
        }
    }

    pub fn simultaneous_traverse(&self, seq: InstructionSeq) -> Result<usize, usize> {
        let seq = seq.0;
        if !seq.is_empty() {
            let mut seq = seq.into_iter().cycle();

            let mut nodes: Vec<_> = self
                .trees
                .iter()
                .filter(|x| x.id.ends_with_a())
                .map(|x| &x.id)
                .collect();
            let mut n: usize = 0;
            while !nodes.iter().all(|x| x.ends_with_z()) {
                let insn = seq.next().unwrap();
                n += 1;
                for node in nodes.iter_mut() {
                    match self.branch(insn, node) {
                        Some(next) => {
                            *node = next;
                        }
                        None => return Err(n),
                    }
                }
            }
            Ok(n)
        } else {
            Err(0)
        }
    }
}

pub fn seq_network_from_path<T: AsRef<Path>>(path: T) -> Result<(InstructionSeq, Network), String> {
    let s = fs::read_to_string(path.as_ref()).map_err(|e| e.to_string())?;
    if let Some((lhs, rhs)) = s.split_once("\n\n") {
        let lhs = lhs.parse::<InstructionSeq>()?;
        let rhs = rhs.parse::<Network>()?;
        Ok((lhs, rhs))
    } else {
        Err(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;
    /*
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    */

    #[test]
    fn instruction_seq_from_str() {
        let lhs = "LLRRL".parse::<InstructionSeq>().unwrap();
        assert_eq!(lhs, InstructionSeq(vec![L, L, R, R, L]));
    }

    #[test]
    fn node_from_str() {
        let lhs = "XYZ".parse::<Node>().unwrap();
        assert_eq!(lhs, Node::from(['X', 'Y', 'Z']));
    }

    #[test]
    fn network_from_str() {
        let s = "\
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let lhs = s.parse::<Network>().unwrap();
        let rhs = Network::from(vec![
            Tree::new(
                Node::from(['A', 'A', 'A']),
                Node::from(['B', 'B', 'B']),
                Node::from(['B', 'B', 'B']),
            ),
            Tree::new(
                Node::from(['B', 'B', 'B']),
                Node::from(['A', 'A', 'A']),
                Node::from(['Z', 'Z', 'Z']),
            ),
            Tree::new(
                Node::from(['Z', 'Z', 'Z']),
                Node::from(['Z', 'Z', 'Z']),
                Node::from(['Z', 'Z', 'Z']),
            ),
        ]);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn traverse_works() {
        let network = Network::from(vec![
            Tree::new(
                Node::from(['A', 'A', 'A']),
                Node::from(['B', 'B', 'B']),
                Node::from(['B', 'B', 'B']),
            ),
            Tree::new(
                Node::from(['B', 'B', 'B']),
                Node::from(['A', 'A', 'A']),
                Node::from(['Z', 'Z', 'Z']),
            ),
            Tree::new(
                Node::from(['Z', 'Z', 'Z']),
                Node::from(['Z', 'Z', 'Z']),
                Node::from(['Z', 'Z', 'Z']),
            ),
        ]);
        let inst_set = InstructionSeq(vec![L, L, R]);
        let entry = Node::from(['A', 'A', 'A']);
        let exit = Node::from(['Z', 'Z', 'Z']);
        assert_eq!(network.traverse(inst_set, entry, exit).unwrap(), 6);
    }

    #[test]
    fn simultaneous_traverse_works() {
        let s = "\
DDA = (DDB, XXX)
DDB = (XXX, DDZ)
DDZ = (DDB, XXX)
FFA = (FFB, XXX)
FFB = (FFC, FFC)
FFC = (FFZ, FFZ)
FFZ = (FFB, FFB)
XXX = (XXX, XXX)";
        let network = s.parse::<Network>().unwrap();
        let lhs = network
            .simultaneous_traverse(InstructionSeq(vec![L, R]))
            .unwrap();
        assert_eq!(lhs, 6);
    }
}
