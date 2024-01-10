use crate::grid::*;
use std::cell::RefCell;
use std::collections::{hash_map::DefaultHasher, HashSet};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn inverse(&self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}
use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Elem {
    Empty,      // '.'
    MirrorUp,   // '/'
    MirrorDown, // '\'
    SplitVert,  // '|'
    SplitHorz,  // '-'
}
use Elem::*;

impl Elem {
    pub fn redirect(&self, tracer_dir: Direction) -> (Direction, Option<Direction>) {
        match (self, tracer_dir) {
            (Empty, x) | (SplitVert, x @ (Up | Down)) | (SplitHorz, x @ (Left | Right)) => {
                (x, None)
            }
            (SplitVert, Left | Right) => (Up, Some(Down)),
            (SplitHorz, Up | Down) => (Left, Some(Right)),
            (MirrorUp, Right) => (Up, None),
            (MirrorUp, Left) => (Down, None),
            (MirrorUp, Down) => (Left, None),
            (MirrorUp, Up) => (Right, None),
            (MirrorDown, Right) => (Down, None),
            (MirrorDown, Left) => (Up, None),
            (MirrorDown, Up) => (Left, None),
            (MirrorDown, Down) => (Right, None),
        }
    }
}

impl TryFrom<char> for Elem {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Empty),
            '/' => Ok(MirrorUp),
            '\\' => Ok(MirrorDown),
            '|' => Ok(SplitVert),
            '-' => Ok(SplitHorz),
            _ => Err(c.to_string()),
        }
    }
}
impl From<Elem> for char {
    fn from(e: Elem) -> char {
        match e {
            Empty => '.',
            MirrorUp => '/',
            MirrorDown => '\\',
            SplitVert => '|',
            SplitHorz => '-',
        }
    }
}
impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contraption(Grid<Elem>);

impl Contraption {
    pub fn ray_trace(&self) -> Grid<bool> {
        let (n_rows, n_cols) = self.0.shape();
        let mut rhs = Grid::new_default(n_rows, n_cols);
        rhs[(0, 0)] = true;
        let grid = Rc::new(RefCell::new(rhs));
        {
            let mut tracer = Tracer {
                current: (0, 0),
                dir: Right,
                layout: &self.0,
                energized: Rc::clone(&grid),
                cache: HashSet::new(),
            };
            tracer.trace();
        }
        Rc::into_inner(grid).unwrap().into_inner()
    }
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
    pub fn count_energized(&self) -> usize {
        self.ray_trace()
            .inner
            .into_iter()
            .fold(0usize, |acc, x| acc + x as usize)
    }
}

impl FromStr for Contraption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Contraption(s.parse::<Grid<Elem>>()?))
    }
}

impl fmt::Display for Contraption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Tracer<'a> {
    current: (usize, usize),
    dir: Direction,
    layout: &'a Grid<Elem>,
    energized: Rc<RefCell<Grid<bool>>>,
    // Cycle detection in absence of any exploitable knowledge will
    // is most easily implemented by accumulating a cache of visited
    // states (each state a hash of position and direction).
    //
    // Moreover, the nature of the problem is such that
    // a single marker per position is not sufficient -- that is, if
    // one were to propose that a stack of "paused" tracers (rather than running
    // the trace immediately after creation) be created, so that we
    // might employ a single mark per position (on a third grid, which we reset
    // to a null state after each trace has run to completion).
    // There are a few reasons that it is insufficient:
    // 1) one can draw a pattern which cycles through a single point through
    //    more than one direction (think a crudely drawn infinity symbol).
    //    We might overcome this by having a marker for each possible direction
    //    (thus, 4 marks per position).
    // 2) a clean state grid may prevent detection of certain patterns;
    //    As each trace begins from a clean grid, it is possible that some of
    //    the erased history would have been necessary to detect the cycle.
    //
    // Furthermore, a third grid is not necessarily fewer heap allocations,
    // as we must store all but 1 of the paused traces on the heap until
    // it is ready to run -- thus, we must have sufficient memory to do so,
    // whereas if we use a tracer-owned cache, said cache is freed when
    // the tracer expires.
    cache: HashSet<u64>,
}
impl Tracer<'_> {
    pub fn move_up(&mut self) -> bool {
        if self.current.0 != 0 {
            self.current.0 -= 1;
            true
        } else {
            false
        }
    }
    pub fn move_down(&mut self) -> bool {
        let new = self.current.0 + 1;
        if new < self.layout.n_rows() {
            self.current.0 = new;
            true
        } else {
            false
        }
    }
    pub fn move_left(&mut self) -> bool {
        if self.current.1 != 0 {
            self.current.1 -= 1;
            true
        } else {
            false
        }
    }
    pub fn move_right(&mut self) -> bool {
        let new = self.current.1 + 1;
        if new < self.layout.n_cols() {
            self.current.1 = new;
            true
        } else {
            false
        }
    }
    pub fn try_move(&mut self, dir: Direction) -> bool {
        let state = match dir {
            Up => {
                self.dir = Up;
                self.move_up()
            }
            Down => {
                self.dir = Down;
                self.move_down()
            }
            Left => {
                self.dir = Left;
                self.move_left()
            }
            Right => {
                self.dir = Right;
                self.move_right()
            }
        };
        if state {
            self.energized.borrow_mut()[self.current] = true;
            true
        } else {
            false
        }
    }
    pub fn advance(&mut self) -> (bool, Option<Tracer<'_>>) {
        // Simple cycle detection using position and direction
        let mut state = DefaultHasher::new();
        self.current.hash(&mut state);
        self.dir.hash(&mut state);
        // If this is not an obvious cycle, proceed
        if self.cache.insert(state.finish()) {
            match self.layout[self.current].redirect(self.dir) {
                (first, Some(second)) => {
                    let mut rhs = Tracer {
                        current: self.current.clone(),
                        dir: self.dir.clone(),
                        layout: &*self.layout,
                        energized: Rc::clone(&self.energized),
                        cache: self.cache.clone(),
                    };
                    let rhs = if rhs.try_move(second) {
                        Some(rhs)
                    } else {
                        None
                    };
                    (self.try_move(first), rhs)
                }
                (first, None) => (self.try_move(first), None),
            }
        } else {
            (false, None)
        }
    }
    pub fn trace(&mut self) {
        loop {
            match self.advance() {
                (true, None) => (),
                (false, None) => break,
                (true, Some(mut branch)) => {
                    branch.trace();
                }
                (false, Some(mut branch)) => {
                    branch.trace();
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn from_str() {
        let lhs = TEST.parse::<Contraption>().unwrap().to_string();
        assert_eq!(lhs, TEST);
    }

    fn println_trace(grid: &Grid<bool>) {
        let (n_rows, n_cols) = grid.shape();
        for i in 0..n_rows {
            for j in 0..n_cols {
                let c = if grid[(i, j)] { '#' } else { '.' };
                print!("{}", c);
            }
            print!("{}", '\n');
        }
    }

    static SIMPLE: &str = r#"...|.
...|.
.\.-\
.|..|
.\--/"#;
    #[test]
    fn noncyclic_trace() {
        let x = SIMPLE.parse::<Contraption>().unwrap();
        let grid = x.ray_trace();
        println_trace(&grid);
        let energized = grid.inner.into_iter().fold(0u8, |acc, x| acc + x as u8);
        assert_eq!(energized, 4 + 2 + 5 + 2 + 4, "\n{}", x);
    }
    #[test]
    fn cyclic_trace() {
        let x = TEST.parse::<Contraption>().unwrap();
        let grid = x.ray_trace();
        println_trace(&grid);
        let energized = grid.inner.into_iter().fold(0u8, |acc, x| acc + x as u8);
        assert_eq!(energized, 46, "\n{}", x);
    }
}
