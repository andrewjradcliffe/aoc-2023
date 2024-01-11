use crate::grid::*;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::{fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub fn ray_trace(&self) -> Grid<Mark> {
        if self.0.len() == 0 {
            Grid::new_default(0, 0)
        } else {
            let (n_rows, n_cols) = self.0.shape();
            let states = Rc::new(RefCell::new(Grid::new_default(n_rows, n_cols)));
            {
                let mut tracer = Tracer {
                    current: (0, 0),
                    dir: Right,
                    layout: &self.0,
                    states: Rc::clone(&states),
                };
                tracer.trace();
            }
            Rc::into_inner(states).unwrap().into_inner()
        }
    }
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Self, String> {
        let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
        s.parse::<Self>()
    }
    pub fn count_energized(&self) -> usize {
        if self.0.len() == 0 {
            0
        } else {
            self.ray_trace()
                .inner
                .into_iter()
                .fold(0usize, |acc, x| acc + x.any() as usize)
        }
    }
    fn ray_trace_imp(&self, i: usize, j: usize, dir: Direction, states: Rc<RefCell<Grid<Mark>>>) {
        states.borrow_mut().inner.iter_mut().for_each(|x| x.reset());
        let mut tracer = Tracer {
            current: (i, j),
            dir,
            layout: &self.0,
            states,
        };
        tracer.trace();
    }
    pub fn maximum_energized(&self) -> usize {
        if self.0.len() == 0 {
            0
        } else {
            let (n_rows, n_cols) = self.0.shape();
            let states = Rc::new(RefCell::new(Grid::new_default(n_rows, n_cols)));
            let mut mx: usize = 0;
            let right = n_cols - 1;
            let bottom = n_rows - 1;
            for (dir, j) in [(Right, 0), (Left, right)] {
                for i in 0..n_rows {
                    self.ray_trace_imp(i, j, dir, Rc::clone(&states));
                    let total = states
                        .borrow()
                        .inner
                        .iter()
                        .fold(0usize, |acc, x| acc + x.any() as usize);
                    mx = mx.max(total);
                }
            }
            for (dir, i) in [(Down, 0), (Up, bottom)] {
                for j in 0..n_cols {
                    self.ray_trace_imp(i, j, dir, Rc::clone(&states));
                    let total = states
                        .borrow()
                        .inner
                        .iter()
                        .fold(0usize, |acc, x| acc + x.any() as usize);
                    mx = mx.max(total);
                }
            }
            mx
        }
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

/*
Yes, we could implement this 16-state wonder on a `u8`, but
for ease of use we'll allow 3 more bytes.
*/
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Mark {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl Mark {
    pub fn mark(&mut self, dir: Direction) {
        match dir {
            Up => self.up = true,
            Down => self.down = true,
            Left => self.left = true,
            Right => self.right = true,
        }
    }
    #[inline]
    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
    }
    #[inline]
    pub fn any(&self) -> bool {
        self.up | self.down | self.left | self.right
    }
}

/*
The `u8` impl would be:

const UP: u8 = 0x08;
const DOWN: u8 = 0x04;
const LEFT: u8 = 0x02;
const RIGHT: u8 = 0x01;
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Mark(u8);
impl Mark {
pub fn is_up(&self) -> bool {
self.0 & UP == UP
    }
    pub fn is_down(&self) -> bool {
        self.0 & DOWN == DOWN
    }
    pub fn is_left(&self) -> bool {
        self.0 & LEFT == LEFT
    }
    pub fn is_right(&self) -> bool {
        self.0 & RIGHT == RIGHT
    }
    pub fn set_up(&mut self) {
        self.0 |= UP
    }
    pub fn set_down(&mut self) {
        self.0 |= DOWN
    }
    pub fn set_left(&mut self) {
        self.0 |= LEFT
    }
    pub fn set_right(&mut self) {
        self.0 |= RIGHT
    }
    pub fn any(&self) -> bool {
        self.0 != 0
    }
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}
*/

/*

* First thought

Cycle detection in absence of any exploitable knowledge will
is most easily implemented by accumulating a cache of visited
states (each state a hash of position and direction).

Moreover, the nature of the problem is such that
a single marker per position is not sufficient -- that is, if
one were to propose that a stack of "paused" tracers (rather than running
the trace immediately after creation) be created, so that we
might employ a single mark per position (on a third grid, which we reset
to a null state after each trace has run to completion).
There are a few reasons that it is insufficient:
1) one can draw a pattern which cycles through a single point through
   more than one direction (think a crudely drawn infinity symbol).
   We might overcome this by having a marker for each possible direction
   (thus, 4 marks per position).
2) a clean state grid may prevent detection of certain patterns;
   As each trace begins from a clean grid, it is possible that some of
   the erased history would have been necessary to detect the cycle.

Furthermore, a third grid is not necessarily fewer heap allocations,
as we must store all but 1 of the paused traces on the heap until
it is ready to run -- thus, we must have sufficient memory to do so,
whereas if we use a tracer-owned cache, said cache is freed when
the tracer expires.

* On second thought

Analysis of the complexity of the cache-based approach indicates
that it implies an enormous amount of space in the worst case --
O((m + p) * 2^q) where m is the cycle length, p is the path length leading
to the cycle, and q is the number of branches; this assumes that
    - every path ends in a cycle
    - every path and cycle are the same length
    - every branch (i.e. `|` or `-`) leads to 2 more branches, and
      that a single point begins the branching sequence, hence,
      the branching pattern is a binary tree
This is obviously a very pessimistic worst-case. If we remove all cycles,
we still have O(p * 2^q) space. Since we are creating a clone of the cache
at each branch, we are not actually computing a number of hashes equivalent
to the storage requirement, but certainly we are doing the work of storing
that many u64s (i.e. memcpy is not free).

There is a simpler way to do this, namely, marking each element as having
been traversed in a given direction. If we try to traverse the element
in a direction which has already been marked, then we know we are about
to embark on a cycle.

Furthermore, there are additional improvements that can be made.
First, one can store marks for redirection elements only, and build
a collection of ((i, j), mark) dynamically, such that the total space
requirement is O(n) where n is the number of redirection marks.
For this particular problem, this is less convenient, as we wish to
count the total number of elements visited, but, if one were interested
in only the vertices, it would be useful.
Second, if one knows in advance that the graph will be traversed multiple
times (e.g. in solving an optimization problem), then the distance
between vertices can be computed a single time for all, such that
an actual walk of the coordinates is performed only once. However,
this requires that one walk each direction for each redirection element;
whether this is worthwhile is determined by the sparsity of the graph --
increasing sparsity makes this more likely to be a beneficial tradeoff.
*/
#[derive(Debug, Clone)]
pub struct Tracer<'a> {
    current: (usize, usize),
    dir: Direction,
    layout: &'a Grid<Elem>,
    states: Rc<RefCell<Grid<Mark>>>,
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
        match dir {
            Up => {
                self.dir = Up;
                if self.states.borrow()[self.current].up {
                    false
                } else {
                    self.states.borrow_mut()[self.current].up = true;
                    self.move_up()
                }
            }
            Down => {
                self.dir = Down;
                if self.states.borrow()[self.current].down {
                    false
                } else {
                    self.states.borrow_mut()[self.current].down = true;
                    self.move_down()
                }
            }
            Left => {
                self.dir = Left;
                if self.states.borrow()[self.current].left {
                    false
                } else {
                    self.states.borrow_mut()[self.current].left = true;
                    self.move_left()
                }
            }
            Right => {
                self.dir = Right;
                if self.states.borrow()[self.current].right {
                    false
                } else {
                    self.states.borrow_mut()[self.current].right = true;
                    self.move_right()
                }
            }
        }
    }
    pub fn advance(&mut self) -> (bool, Option<Tracer<'_>>) {
        // Simple cycle detection using position and direction
        match self.layout[self.current].redirect(self.dir) {
            (first, Some(second)) => {
                let mut rhs = Tracer {
                    current: self.current.clone(),
                    dir: self.dir.clone(),
                    layout: &*self.layout,
                    states: Rc::clone(&self.states),
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

    fn println_trace(grid: &Grid<Mark>) {
        let (n_rows, n_cols) = grid.shape();
        for i in 0..n_rows {
            for j in 0..n_cols {
                let c = if grid[(i, j)].any() { '#' } else { '.' };
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
        let energized = grid
            .inner
            .into_iter()
            .fold(0u8, |acc, x| acc + x.any() as u8);
        assert_eq!(energized, 4 + 2 + 5 + 2 + 4, "\n{}", x);
    }
    #[test]
    fn cyclic_trace() {
        let x = TEST.parse::<Contraption>().unwrap();
        let grid = x.ray_trace();
        println_trace(&grid);
        let energized = grid
            .inner
            .into_iter()
            .fold(0u8, |acc, x| acc + x.any() as u8);
        assert_eq!(energized, 46, "\n{}", x);
    }

    #[test]
    fn maximum_energized() {
        let x = TEST.parse::<Contraption>().unwrap();
        assert_eq!(x.maximum_energized(), 51);
    }
}
