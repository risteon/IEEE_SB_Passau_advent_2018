use std::io::{self, BufRead};

const MINE: char = '*';
const EMPTY: char = '.';


fn f(c: char) -> Option<u8> {
    match c {
        '0'...'9' => Some(c as u8 - '0' as u8),
        EMPTY => Some(0u8),
        MINE => None,
        _ => panic!("Invalid input character"),
    }
}


fn check(grid: &[&[Option<u8>]], r: usize, c: usize, h: usize, w: usize) -> bool {
    if grid[r][c].is_none() {
        return true;
    }
    let mut m = 0u8;
    let r_min = if r > 0 {r - 1} else {0};
    let c_min = if c > 0 {c - 1} else {0};
    let r_max = if r < h - 1 {r + 1} else {r};
    let c_max = if c < w - 1 {c + 1} else {c};

    for r_c in r_min..r_max+1 {
        for c_c in c_min..c_max+1 {
            if r_c == r && c_c == c {
                continue;
            }
            if grid[r_c][c_c].is_none() {
                m += 1;
            }
        }
    }
    m == grid[r][c].unwrap()
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let v : Vec<u32> = iterator.next().unwrap().unwrap()
        .split(" ").map(|x| x.parse::<u32>().unwrap()).collect();
    let w = v[0];
    let h = v[1];

    // build 2D array
    let grid_raw = {
        let mut grid_raw: Vec<Option<u8>> = vec![None; (w * h) as usize];
        {
            let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(w as usize).collect();
            let grid: &mut [&mut [_]] = grid_base.as_mut_slice();
            let mut h_counter = 0usize;

            for line in iterator {
                let s = line.unwrap();
                assert_eq!(s.chars().count(), w as usize);
                let v: Vec<Option<u8>> = s.chars().map(f).collect();
                grid[h_counter][..].clone_from_slice(&v);
                h_counter += 1;
            }
        }
        grid_raw
    };

    let grid_base: Vec<_> = grid_raw.as_slice().chunks(w as usize).collect();
    let grid: &[&[_]] = grid_base.as_slice();
    for (r, row) in grid.iter().enumerate() {
        for (c, _col) in row.iter().enumerate() {
            if !check(grid, r, c, h as usize, w as usize) {
                println!("Nein");
                return;
            }
        }
    }
    println!("Ja");
}
