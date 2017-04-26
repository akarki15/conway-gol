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
    let mut grid = Vec::new();
    for i in 1..40 {
        let mut row = Vec::new();
        for i in 1..40 {
            let c: Cell = Cell { alive: true };
            row.push(c);
        }
        grid.push(row);
    }
    grid[1][1].alive = false;
    for i in grid.iter() {
        for j in i.iter() {
            print!("{0}", j);
        }
        print!("\n");
    }
}

fn get_neighbors(i: i32, j: i32, n: i32) -> Vec<Vec<i32>> {
    let mut neighbors = Vec::new();
    neighbors.push(vec![i - 1, j]);
    neighbors.push(vec![i, j - 1]);
    neighbors.push(vec![i - 1, j - 1]);
    neighbors.push(vec![i, j + 1]);
    neighbors.push(vec![i + 1, j + 1]);
    neighbors.push(vec![i + 1, j]);
    neighbors.push(vec![i + 1, j - 1]);
    neighbors.push(vec![i - 1, j + 1]);

    let mut legitNeighbor = Vec::new();
    for neighbor in neighbors.iter() {
        if neighbor[0] < 0 || neighbor[1] < 0 || neighbor[0] >= n || neighbor[1] >= n {
            continue;
        }
        legitNeighbor.push(vec![neighbor[0], neighbor[1]]);
    }
    legitNeighbor
}


#[cfg(test)]
mod test {
    use super::get_neighbors;

    #[test]
    fn test_get_neighbors() {
        let mut neighbors = get_neighbors(0, 0, 3);
        assert_eq!(3, neighbors.len());

        neighbors = get_neighbors(1, 1, 3);
        assert_eq!(8, neighbors.len());
    }
}
