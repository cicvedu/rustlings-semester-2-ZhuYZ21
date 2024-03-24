use std::f32::consts::PI;

fn main() {
    let pi = PI;
    let radius = 5.00f32;

    let area = pi * radius.powf(2.0);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}