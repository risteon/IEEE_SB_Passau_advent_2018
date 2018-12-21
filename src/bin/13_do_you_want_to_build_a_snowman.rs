use std::io::{self, BufRead};


fn volume(height: f64) -> f64{
    fn sphere(radius: f64) -> f64 {
        radius.powi(3) * 4.0f64 / 3.0f64 * std::f64::consts::PI
    }
    let r1 = height * 0.5/4.5;
    let r2 = height * 0.75/4.5;
    let r3 = height * 1.0/4.5;
    sphere(r1) + sphere(r2) + sphere(r3)
}


fn main() {

    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();

    let v : Vec<f64> =
        iterator.next().unwrap().unwrap()
            .split(" ").map(|x| x.parse::<f64>().unwrap()).collect();
    assert_eq!(v.len(), 3);
    let height = iterator.next().unwrap().unwrap().parse::<f64>().unwrap();
    let snow = v.iter().fold(1.0f64, |prod, x| prod * *x);
    match snow >= volume(height) {
        true => println!("Let's do it!"),
        false => println!("Let it go!"),
    }
}
