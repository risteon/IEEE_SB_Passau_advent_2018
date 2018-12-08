use std::io::{self, BufRead};

fn turn(n: usize, s: char, t: char, a: char) {
    if n == 0 { return; }
    turn(n - 1, s, a, t);
    println!("{} -> {}", s, t);
    turn(n - 1, a, t, s);
}

fn main() {
    let l = io::stdin();
    turn(l.lock().lines().count(), 'A', 'Z', 'V');
}
