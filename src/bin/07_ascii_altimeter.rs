use std::io::{self, BufRead};


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let _no_lines = iterator.next().unwrap().unwrap().parse::<usize>().unwrap();


    for line in iterator {
        let s = line.unwrap();
        let mut counter = 0i32;
        for c in s.chars() {
            match c {
                '/' => counter += 1,
                '\\' => {if counter == 0 {counter = 1; break;} counter -= 1;},
                _ => panic!("invalid"),
            };
        }
        match counter {
            0 => println!{"1"},
            _ => println!{"0"},
        };
    }

}
