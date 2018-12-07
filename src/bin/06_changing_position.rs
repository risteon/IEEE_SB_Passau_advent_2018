use std::io::{self, BufRead};
use std::option::Option;


fn work(v: &Vec<u32>, d: u32) -> Option<u32> {
    let n: u32 = v.len() as u32;
    let mut dist = 0u32;
    let mut b = vec![false; v.len()];

    for (c, v) in v.iter().cloned().enumerate() {
        assert_eq!(v <= n, true);
        if v > c as u32 + 1 + d {
            return None;
        }
        for i in 0..(v - 1) as usize {
            if !b[i] {
                dist += 1;
            }
        }
        b[(v - 1) as usize] = true;
    }
    Some(dist)
}


fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let n = iterator.next().unwrap().unwrap().parse::<u32>().unwrap();
    let v : Vec<u32> =
        iterator.next().unwrap().unwrap()
                .split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

    assert_eq!(v.len(), n as usize);
    match work(&v, 2u32) {
        Some(x) => println!("{}", x),
        None => println!("CHAOS"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial() {
        for c in 2u32..20u32 {
            let v: Vec<u32> = (1u32..c).collect();
            assert_eq!(work(&v, 1u32), Some(0));
        }
    }

    #[test]
    fn test_cases() {
        assert_eq!(work(&vec![2u32, 1u32, 5u32, 3u32, 4u32], 2u32), Some(3));
        assert_eq!(work(&vec![2u32, 5u32, 1u32, 3u32, 4u32], 2u32), None);

        assert_eq!(work(&vec![3u32, 1u32, 2u32], 2u32), Some(2));
        assert_eq!(work(&vec![3u32, 2u32, 1u32], 2u32), Some(3));
    }
}
