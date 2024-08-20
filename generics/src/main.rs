use num_traits::{ToPrimitive, Float};

fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

// implements the float trait.
fn solve_generic<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}



fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    //let a_f64: f64 = a as f64;

    let a_f64: f64 = a.to_f64().unwrap();

    println!("{}", solve(a_f64, b));
    println!("{}", solve_generic(a_f64, b));

}
