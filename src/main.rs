type Field = Vec<Vec<Option<u32>>>;

fn print_field(field: &Field) {
    for y in 0..9 {
        for x in 0..9 {
            if let Some(v) = field[y][x] {
                print!("{} ", v);
            } else {
                print!("  ")
            }
        }
        println!("")
    }
}

fn read_line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned()
}

fn dfs(board: &mut sudoku::Sudoku, results: &mut Vec<Field>) {
    match board.find_empty() {
        Some(pt) => {
            let can_use = board.find_choices(&pt);
            for &val in &can_use {
                board.put(&pt, val);
                dfs(board, results);
                board.reset(&pt);
            }
        }
        None => {
            results.push(board.get().clone());
        }
    }
}

fn main() {
    let mut field: Field = vec![vec![None; 9]; 9];
    for y in 0..9 {
        for (i, c) in read_line().chars().enumerate() {
            field[y][i] = c.to_digit(10);
        }
    }

    let mut board = sudoku::Sudoku(field);
    let mut results: Vec<Field> = Default::default();

    dfs(&mut board, &mut results);

    if results.len() == 0 {
        println!("No solutions.")
    } else if results.len() > 1 {
        println!("More than one solution.")
    } else {
        print_field(&results[0]);
    }
}

mod sudoku {
    use super::Field;
    use std::collections::HashSet;

    pub struct Point {
        pub x: usize,
        pub y: usize,
    }

    pub struct Sudoku(pub Field);

    impl Sudoku {
        pub fn get(&self) -> &Field {
            &self.0
        }

        pub fn put(&mut self, pt: &Point, val: u32) {
            self.0[pt.y][pt.x] = Some(val);
        }

        pub fn reset(&mut self, pt: &Point) {
            self.0[pt.y][pt.x] = None;
        }

        pub fn find_empty(&self) -> Option<Point> {
            let field = &self.0;
            for y in 0..9 {
                for x in 0..9 {
                    if field[y][x].is_none() {
                        return Some(Point { x, y });
                    }
                }
            }
            None
        }

        pub fn find_choices(&self, pt: &Point) -> HashSet<u32> {
            let field = &self.0;
            let mut can_use: HashSet<u32> = (1..=9).collect();

            // Remove values existing in the row
            (0usize..9).map(|i| field[pt.y][i]).flatten().for_each(|n| {
                can_use.remove(&n);
            });
            // Remove values existing in the column
            (0usize..9).map(|i| field[i][pt.x]).flatten().for_each(|n| {
                can_use.remove(&n);
            });
            // Remove values existing in the box
            let cx = pt.x / 3 * 3 + 1;
            let cy = pt.y / 3 * 3 + 1;
            for y in cy - 1..=cy + 1 {
                for x in cx - 1..=cx + 1 {
                    if let Some(n) = field[y][x] {
                        can_use.remove(&n);
                    }
                }
            }
            can_use
        }
    }
}
