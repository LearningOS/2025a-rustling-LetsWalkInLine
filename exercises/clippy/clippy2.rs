// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     while let Some(x) = option {
//         res += x;
//     }
//     println!("{}", res);
// }

use std::f32;
use std::f32::consts::PI;

fn main() {
    let pi = PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
