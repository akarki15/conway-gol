use std::fmt;
use piston_window::*;

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
 let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    print!("\n");
    for i in grid.iter() {
        for j in i.iter() {
            print!("{0}", j);
        }
        print!("\n");
    }
}

fn change_state(grid: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut nextGrid = Vec::new();

    for i in 0..grid.len() {
        let mut row = Vec::new();
        for j in 0..grid[i].len() {
            let neighbors = get_neighbors(i as i32, j as i32, grid.len() as i32);
            let newState = cell_state(grid[i][j].alive, &grid, neighbors);
            let c: Cell = Cell { alive: newState };
            row.push(c);
        }
        nextGrid.push(row);
    }
    nextGrid
}

fn cell_state(state: bool, grid: &Vec<Vec<Cell>>, neighbors: Vec<Vec<i32>>) -> bool {
    let mut numAliveCells = 0;
    for neighbor in neighbors {
        let x = neighbor[0] as usize;
        let y = neighbor[1] as usize;
        if grid[x][y].alive {
            numAliveCells += 1;
        }
    }

    if !state {
        if numAliveCells == 3 {
            return true;
        } else {
            return false;
        }
    }
    if numAliveCells < 2 || numAliveCells > 3 {
        return false;
    }
    return true;
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
