use std::io::{self, BufRead};


fn parse_value(input: &str, base: u32) -> u128 {
    let mut v = 0u128;
    let l = input.chars().count() - 1;
    for (i, c) in input.chars().enumerate() {
        let f = (base as u128).pow((l - i) as u32);
        if c.is_ascii_uppercase() {
            v += f * (c as u128 - 'A' as u128 + 10);
        }
        else if c.is_ascii_digit() {
            v += f * (c as u128 - '0' as u128);
        }
        else {
            panic!("invalid input")
        }
    }
    v
}

fn write_value(mut value: u128, base: u32) -> String {
    let mut l = 0usize;
    while (base as u128).pow(l as u32) - 1 < value {
        l += 1;
    }
    if value == 0 {
        l = 1;
    }

    let mut s = "".to_string();

    for i in 0..l {
        let f = (base as u128).pow((l - i - 1) as u32);
        let v = value / f;
        assert_eq!(v < base as u128, true);
        value -= v * f;

        if v > 9 {
            s.push(('A' as u8 + (v - 10) as u8) as char);
        }
        else {
            s.push(('0' as u8 + v as u8) as char);
        }
    }
    s
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let _n = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();

    for line in iterator {

        let s = line.unwrap();
        let split = s.split(" ");
        let v : Vec<&str> = split.collect();
        assert_eq!(v.len(), 3);
        let base_from = v[1].parse::<u32>().unwrap();
        let base_to = v[2].parse::<u32>().unwrap();

        let value = parse_value(v[0], base_from);
        println!("{}", write_value(value, base_to));
    }
}
