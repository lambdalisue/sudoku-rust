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

fn main() {
    let mut field: Field = vec![vec![None; 9]; 9];
    for y in 0..9 {
        for (i, c) in read_line().chars().enumerate() {
            field[y][i] = c.to_digit(10);
        }
    }

    print_field(&field);
}
