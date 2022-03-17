use rand::random;
use rayon::prelude::*;

fn main() {
    let n_jobs: usize = 10000000;

    let inside = (0..n_jobs)
        .into_par_iter()
        .map(|_| {
            let x = random::<f64>();
            let y = random::<f64>();
            ((x * x) + (y * y) < 1.0) as usize
        })
        .reduce(|| 0, |a, b| a + b);

    let pi = 4.0 * inside as f64 / n_jobs as f64;
    println!("Num inside: {} -> pi: {}", inside, pi);
}
