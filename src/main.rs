use std::fmt;

struct Cell {
    alive: bool,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.alive {
            write!(f, "{}", "x")
        } else {
            write!(f, "{}", "o")
        }
    }
}

fn main() {
    let c: Cell = Cell { alive: true };
    let c2: Cell = Cell { alive: false };
    let mut grid: [Cell; 1] = [c];
    grid[0] = c2;
    println!("{0}", grid[0]);
}
