use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections;


fn decode(roman: &String,
          mapping: &HashMap<char, u32, collections::hash_map::RandomState>) -> u32 {
    assert!(!roman.is_empty());

    let mut numbers = Vec::new();
    numbers.reserve(roman.len());
    for c in roman.chars() {
        numbers.push(mapping.get(&c));
    }

    let mut sum = 0u32;
    let mut c = 0 as usize;
    while c < numbers.len() {
        if c < numbers.len() - 1 && numbers[c] < numbers[c+1] {
            sum += numbers[c + 1].unwrap();
            sum -= numbers[c].unwrap();
            c += 1;
        }
        else {
            sum += numbers[c].unwrap();
        }
        c += 1;
    }
    sum
}

fn get_standard_mapping() -> HashMap<char, u32, collections::hash_map::RandomState> {
    // create dict
    let mut roman_numeral = HashMap::new();
    roman_numeral.insert('I', 1u32);
    roman_numeral.insert('V', 5u32);
    roman_numeral.insert('X', 10u32);
    roman_numeral.insert('L', 50u32);
    roman_numeral.insert('C', 100u32);
    roman_numeral.insert('D', 500u32);
    roman_numeral.insert('M', 1000u32);
    roman_numeral
}

fn main() -> io::Result<()> {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line_no_testcases = iterator.next().unwrap().unwrap();
    let _n = line_no_testcases.parse::<u32>().unwrap();

    // create dict
    let roman_numeral = get_standard_mapping();

    for line in iterator {
        println!("{}", decode(&line.unwrap(), &roman_numeral))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mapping = get_standard_mapping();
        assert_eq!(decode(&"X".to_string(), &mapping), 10);
        assert_eq!(decode(&"D".to_string(), &mapping), 500);
        assert_eq!(decode(&"XIV".to_string(), &mapping), 14);
        assert_eq!(decode(&"XXXIX".to_string(), &mapping), 39);
        assert_eq!(decode(&"MMXVIII".to_string(), &mapping), 2018);
        assert_eq!(decode(&"MDCCLXXVI".to_string(), &mapping), 1776);
        assert_eq!(decode(&"MCMXC".to_string(), &mapping), 1990);
        assert_eq!(decode(&"MCMXCIX".to_string(), &mapping), 1999);
    }
}
