use crate::garden::vegetables::Asparagus;

pub mod garden;


fn evaluate(x: f32) -> f32 {
    println!("{}", x.to_bits());
    let result = (x - 16.0).powf(2.0) / 16.0 + 16.0;
    return result;
}

fn main() {
    // println! is a macro: https://stackoverflow.com/questions/29611387/why-does-the-println-function-use-an-exclamation-mark-in-rust
    println!("{}", evaluate(17.0));

    let plant = Asparagus {};
    println!("{:?}!", plant);
}
