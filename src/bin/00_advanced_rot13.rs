use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line_no_testcases = iterator.next().unwrap().unwrap();
    let _n = line_no_testcases.parse::<u32>().unwrap();


    for line in iterator {
        let l = line.unwrap();
        let input_vec: Vec<char> = l.chars().collect();

        let mut translated_vec = Vec::new();
        translated_vec.reserve(input_vec.len());
        for c in input_vec {
            assert!(c.is_ascii_lowercase());
            let a = ((c as u8 + 13 - 'a' as u8) % 26 + 'a' as u8) as char;
            translated_vec.push(a);
        }
        let translated_string: String = translated_vec.into_iter().collect();
        println!("{}", translated_string);
    }

    Ok(())
}
