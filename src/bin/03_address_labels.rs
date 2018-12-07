use std::io::{self, BufRead};

const CHAR: char = '*';

fn print_header_footer(linewidth: usize) {
    println!("{}",
             std::iter::repeat(CHAR).take(linewidth + 2).collect::<String>());
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let linewidth_str = iterator.next().unwrap().unwrap();
    let linewidth = linewidth_str.parse::<usize>().unwrap();
    let _lines = iterator.next().unwrap().unwrap();

    print_header_footer(linewidth);

    for line in iterator {
        let s = line.unwrap();
        let mut split = s.split(" ");
        let vec = split.collect::<Vec<&str>>();

        let mut l: usize = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < vec.len() {
            while i < vec.len() {
                if vec[i].len() > linewidth {
                    panic!("Single word to long.");
                }
                l += vec[i].chars().count();
                // check if next would fit
                if i < vec.len() - 1 && l + vec[i+1].chars().count() + 1 > linewidth {
                    break;
                }
                // space
                l += 1;
                i += 1;
            }
            i += 1;
            let a = std::cmp::min(i, vec.len());
            //println!("interval {} to {}", j, a);
            let o = vec[j..a].join(" ");
            let space = linewidth - o.chars().count();
            let space_left = space / 2;
            let space_right = space - space_left;
            println!("{}{}{}{}{}",
                     CHAR,
                     std::iter::repeat(' ').take(space_left).collect::<String>(),
                     o,
                     std::iter::repeat(' ').take(space_right).collect::<String>(),
                     CHAR);
            j = i;
            l = 0;
        }
    }

    print_header_footer(linewidth);
}
