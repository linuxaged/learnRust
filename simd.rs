#![feature(core_simd)]
use std::simd::f32x4;
use std::ops::{Add, Sub, Mul};

fn main() {
    use std::simd::f32x4;
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b = f32x4(1.0, 2.0, 3.0, 4.0);
    println!("{:?}", a * b);
}