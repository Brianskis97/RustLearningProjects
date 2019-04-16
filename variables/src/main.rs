const MAX_NUM: u32 = 100_000;
fn main() {
    let mut spaces = "       ";
    let mut x = spaces.len();
    spaces = spaces.trim();
    let mut y = spaces.len();
    print!("oh no is immut: {}\n{}\n", x, y);
}
