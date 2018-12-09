use std::io::{self, Read};

fn main() {
    let prob = [
        0.0651, // a
        0.0189, // b
        0.0306, // c
        0.0508, // d
        0.1740, // e
        0.0166, // f
        0.0301, // g
        0.0476, // h
        0.0755, // i
        0.0027, // j
        0.0121, // k
        0.0344, // l
        0.0252, // m
        0.0978, // n
        0.0251, // o
        0.0079, // p
        0.0002, // q
        0.0700, // r
        0.0758, // s
        0.0615, // t
        0.0435, // u
        0.0067, // v
        0.0189, // w
        0.0003, // x
        0.0004, // y
        0.0113, // z
    ];
    // read input into string
    let stdin = io::stdin();
    let line = {
        let mut line = String::new();
        stdin.lock().read_to_string(&mut line).ok();
        line
    };
    // get occurences of ascii characters
    let mut p = [0u32; 26];
    for c in line.chars() {
        if c.is_ascii_lowercase() {
            p[c as usize - 'a' as usize] += 1;
        }
        else if c.is_ascii_uppercase() {
            p[c as usize - 'A' as usize] += 1;
        }
    }
    // calculate correlation of prob with p
    let mut v = [0.0; 26];
    for i in 0..v.len() {
        let head = &prob[..i];
        let tail = &prob[i..];
        v[i] = p.iter().cloned()
            .zip(tail.iter().cloned().chain(head.iter().cloned()))
            .map(|(x, y)| x as f64 * y).sum();
    }
    // get maximum of correlation
    let mut max_value = 0.0;
    let mut shift = 0usize;
    for (i, value) in v.iter().cloned().enumerate() {
        if value > max_value {
            max_value = value;
            shift = i;
        }
    }
    // print shifted output
    let mut output = String::with_capacity(line.len());
    for c in line.chars() {
        output.push(match c {
            'A'...'Z' => ((c as u8 - 'A' as u8 + shift as u8) % 26 + 'A' as u8) as char,
            'a'...'z' => ((c as u8 - 'a' as u8 + shift as u8) % 26 + 'a' as u8) as char,
            _ => c,
        });
    }
    println!("{}", output);
}
