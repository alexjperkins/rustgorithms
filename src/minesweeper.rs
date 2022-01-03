#![allow(unused)]
use std::fmt;

#[derive(Copy, Clone)]
struct Point(u32, u32);

#[derive(PartialEq, Copy, Clone)]
enum Cell {
    Mine,
    NeighbouringMines(u32),
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min: u32 = 0;
        match self {
            Cell::Mine => write!(f, "*"),
            Cell::NeighbouringMines(how_many) => {
                if how_many > &min { // super annoying...
                    return write!(f, "{}", how_many);
                }
                return write!(f, ".");
            }
            Cell::Empty=> write!(f, "."),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '*' => Cell::Mine,
            '.' => Cell::Empty,
            _ => panic!("Cannot create cell from '{}'", c)
        }
    }
}

struct Minefield {
    field: Vec<MinefieldRow>
}

impl Minefield {
    pub fn fill(&self) -> Self{
        let mut field: Vec<MinefieldRow> = Vec::new();

        for (i, r) in self.field.iter().enumerate() {
            let mut row: Vec<Cell> = Vec::new();

            for (j, c) in r.row.iter().enumerate() {
                let neighbours = self.gather_neighbours(i as u32, j as u32);

                let cell = match c {
                    Cell::Empty => Cell::NeighbouringMines(self.sum_neighbours(neighbours)),
                    Cell::Mine => Cell::Mine,
                    Cell::NeighbouringMines(_) => Cell::NeighbouringMines(self.sum_neighbours(neighbours)),
                };

                row.push(cell);
            }

            field.push(MinefieldRow{ row });
        }

        Self{ field }
    }

    fn sum_neighbours(&self, neighbours: Vec<Cell>) -> u32 {
        neighbours
            .into_iter()
            .filter(|&x| x == Cell::Mine)
            .fold(0, |p, n| p + 1)
    }

    fn gather_neighbours(&self, x: u32, y: u32) -> Vec<Cell> {
        vec![
            Point(x-1, y+1),
            Point(x, y+1),
            Point(x+1, y+1),
            Point(x+1, y),
            Point(x, y-1),
            Point(x-1, y-1),
            Point(x-1, y),
        ]
            .into_iter()
            .filter(|&p| self.cell_exists(p))
            .map(|p| {
                self.cell_from_point(p)
            })
            .collect()
    }

    fn cell_exists(&self, p: Point) -> bool {
        match self.field.get(p.0 as usize) {
            Some(v) =>  match v.row.get(p.1 as usize) {
                Some(&vv) => true,
                _ => false,
            }
            _ => false,
        }
    }

    fn cell_from_point(&self, p: Point) -> Cell {
        match self.field.get(p.0 as usize) {
            Some(v) =>  match v.row.get(p.1 as usize) {
                Some(&vv) => vv,
                _ => panic!("Out of bounds on row: ({}, {})", p.0, p.1),
            }
            _ => panic!("Out of bounds on field: ({}, {})", p.0, p.1),
        }
    }

    fn to_string(&self) -> Vec<String> {
        self.field
            .iter()
            .map(|r| r.to_string())
            .collect()
    }
}

impl From<&[&str]> for Minefield {
    fn from(minefield: &[&str]) -> Self {
        let mut field: Vec<MinefieldRow> = Vec::new();
        for s in minefield.iter().cloned() {
            field.push(MinefieldRow::from(s))
        }

        let s = Self{ field };
        s.fill()
    }
} 

struct MinefieldRow {
    row: Vec<Cell>
}

impl MinefieldRow {
    fn get(&self, i: u32) -> Option<&Cell> {
        self.row.get(i as usize)
    }

    fn to_string(&self) -> String {
        self.row
            .iter()
            .map(|c| c.to_string())
            .collect()
    }
}

impl From<&str> for MinefieldRow {
    fn from(s: &str) -> Self {
        let mut row: Vec<Cell> = Vec::new();
        for c in s.chars() {
            row.push(Cell::from(c));
        }

        Self{row}
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield = Minefield::from(minefield);
    minefield.to_string()
}
