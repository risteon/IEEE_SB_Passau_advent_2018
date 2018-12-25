use std::io::{self, BufRead};

const MAX_DEVIATION: f64 = 90.0f64 / 2.0f64;


fn calc_value(v: Vec<u32>) -> f64 {
    let s: u32 = v.iter().sum();
    let mean = s as f64 / v.len() as f64;
    if v.len() == 1 {
        return mean;
    }

    let sigma: f64 = v.iter().map(|x| (*x as f64 - mean).powi(2)).sum();
    mean - (sigma / (v.len() as f64 - 1.0f64)).sqrt()
}


fn calc_cookie(grid: &[&[u8]], r: usize, c: usize, size_cookie: usize) -> f64 {

    let mut v = vec![0u32; size_cookie * size_cookie];
    for r_c in r..r+size_cookie {
        for c_c in c..c+size_cookie {
            v[(r_c - r) * size_cookie + (c_c - c)] = grid[r_c][c_c] as u32;
        }
    }
    calc_value(v)
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let v : Vec<usize> = iterator.next().unwrap().unwrap()
        .split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
    let size_grid= v[0];
    let size_cookie = v[1];

    // build 2D array
    let grid_raw = {
        let mut grid_raw: Vec<u8> = vec![0; (size_grid * size_grid) as usize];
        {
            let mut grid_base: Vec<_> = grid_raw.as_mut_slice()
                .chunks_mut(size_grid as usize).collect();
            let grid: &mut [&mut [_]] = grid_base.as_mut_slice();
            let mut h_counter = 0usize;

            for line in iterator {
                let s = line.unwrap();
                let v : Vec<u8> = s.split(" ").map(|x| x.parse::<u8>().unwrap()).collect();
                assert_eq!(v.len(), size_grid as usize);
                grid[h_counter][..].clone_from_slice(&v);
                h_counter += 1;
            }
        }
        grid_raw
    };

    let grid_base: Vec<_> = grid_raw.as_slice().chunks(size_grid as usize).collect();
    let grid: &[&[_]] = grid_base.as_slice();

    let mut cookie = (0usize, 0usize);
    let mut value = -10000000.0f64;

    for r in 0..size_grid - size_cookie + 1 {
        for c in 0..size_grid - size_cookie + 1 {
            let v = calc_cookie(grid, r, c, size_cookie);
            if v > value {
                value = v;
                cookie = (r, c);
            }
        }
    }
    println!("{} {}", cookie.1, cookie.0);
}
