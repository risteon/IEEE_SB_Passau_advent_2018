use std::io::{self, BufRead};


fn main() {

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut counter = 0u32;
    let mut height: Vec<u32> = vec![0u32; line.len()];
    for (i, c) in line.chars().enumerate() {
        height[i] = counter;
        match c {
            '/' => counter += 1,
            '\\' => {
                if counter == 0 {panic!("invalid input")}
                counter -= 1;
                height[i] -= 1;
            },
            _ => panic!("invalid input"),
        };
    }

    let ch:Vec<char> = line.chars().collect();
    let h = height.iter().max().unwrap().clone();
    for i in 0u32..h+1 {
        for j in 0..ch.len() {
            if height[j] == h - i {
                print!("{}", ch[j]);
            }
            else {
                print!(" ");
            }
        }
        print!("\n");
    }


}
