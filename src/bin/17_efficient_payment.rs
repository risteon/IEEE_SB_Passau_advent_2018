use std::io::{self, BufRead};


fn process(mut amount: u32) {
    let values = [
        (50000, "500"),
        (20000, "200"),
        (10000, "100"),
        (5000, "50"),
        (2000, "20"),
        (1000, "10"),
        (500, "5"),
        (200, "2"),
        (100, "1"),
        (50, "0.50"),
        (20, "0.20"),
        (10, "0.10"),
        (5, "0.05"),
        (2, "0.02"),
        (1, "0.01"),
    ];

    let mut s = "".to_string();
    let mut i = 0usize;

    while amount != 0u32 {
        if amount >= values[i].0 {
            amount -= values[i].0;
            s.push_str(values[i].1.clone());
            if amount > 0u32 {
                s.push(' ');
            }
        }
        else {
            i += 1;
        }

    }
    println!("{}", s)
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let _n = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();

    for line in iterator {
        let amount = (line.unwrap().parse::<f32>().unwrap() * 100.0).round() as u32;
        process(amount);
    }

}
