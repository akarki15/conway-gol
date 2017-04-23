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
    let mut grid: Vec<Cell> = Vec::new();
    for i in 1..40 {
        let c: Cell = Cell { alive: true };
        grid.push(c);
    }
    for i in 1..40 {
        println!("{0}", grid[0]);
    }
}
