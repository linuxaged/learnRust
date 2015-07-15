#![feature(test)]
extern crate test;
use test::Bencher;
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
struct Matrix4x4 {
    pub m: [f32;16]
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
        	m: [
        		self.m[0] + other.m[0],
        		self.m[1] + other.m[1],
        		self.m[2] + other.m[2],
        		self.m[3] + other.m[3],
        		self.m[4] + other.m[4],
        		self.m[5] + other.m[5],
        		self.m[6] + other.m[6],
        		self.m[7] + other.m[7],
        		self.m[8] + other.m[8],
        		self.m[9] + other.m[9],
        		self.m[10] + other.m[10],
        		self.m[11] + other.m[11],
        		self.m[12] + other.m[12],
        		self.m[13] + other.m[13],
        		self.m[14] + other.m[14],
        		self.m[15] + other.m[15],
        	]
        }
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
        	m: [
        		self.m[0] - other.m[0],
        		self.m[1] - other.m[1],
        		self.m[2] - other.m[2],
        		self.m[3] - other.m[3],
        		self.m[4] - other.m[4],
        		self.m[5] - other.m[5],
        		self.m[6] - other.m[6],
        		self.m[7] - other.m[7],
        		self.m[8] - other.m[8],
        		self.m[9] - other.m[9],
        		self.m[10] - other.m[10],
        		self.m[11] - other.m[11],
        		self.m[12] - other.m[12],
        		self.m[13] - other.m[13],
        		self.m[14] - other.m[14],
        		self.m[15] - other.m[15],
        	]
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
    	Matrix4x4 {
    		m: [
    			self.m[0] * other.m[0] + self.m[1] * other.m[4] + self.m[2] * other.m[8]  + self.m[3] * other.m[12],
    			self.m[0] * other.m[1] + self.m[1] * other.m[5] + self.m[2] * other.m[9]  + self.m[3] * other.m[13],
    			self.m[0] * other.m[2] + self.m[1] * other.m[6] + self.m[2] * other.m[10] + self.m[3] * other.m[14],
    			self.m[0] * other.m[3] + self.m[1] * other.m[7] + self.m[2] * other.m[11] + self.m[3] * other.m[15],
    			self.m[4] * other.m[0] + self.m[5] * other.m[4] + self.m[6] * other.m[8]  + self.m[7] * other.m[12],
    			self.m[4] * other.m[1] + self.m[5] * other.m[5] + self.m[6] * other.m[9]  + self.m[7] * other.m[13],
    			self.m[4] * other.m[2] + self.m[5] * other.m[6] + self.m[6] * other.m[10] + self.m[7] * other.m[14],
    			self.m[4] * other.m[3] + self.m[5] * other.m[7] + self.m[6] * other.m[11] + self.m[7] * other.m[15],
    			self.m[8] * other.m[0] + self.m[9] * other.m[4] + self.m[10] * other.m[8]  + self.m[11] * other.m[12],
    			self.m[8] * other.m[1] + self.m[9] * other.m[5] + self.m[10] * other.m[9]  + self.m[11] * other.m[13],
    			self.m[8] * other.m[2] + self.m[9] * other.m[6] + self.m[10] * other.m[10] + self.m[11] * other.m[14],
    			self.m[8] * other.m[3] + self.m[9] * other.m[7] + self.m[10] * other.m[11] + self.m[11] * other.m[15],
    			self.m[12] * other.m[0] + self.m[13] * other.m[4] + self.m[14] * other.m[8]  + self.m[15] * other.m[12],
    			self.m[12] * other.m[1] + self.m[13] * other.m[5] + self.m[14] * other.m[9]  + self.m[15] * other.m[13],
    			self.m[12] * other.m[2] + self.m[13] * other.m[6] + self.m[14] * other.m[10] + self.m[15] * other.m[14],
    			self.m[12] * other.m[3] + self.m[13] * other.m[7] + self.m[14] * other.m[11] + self.m[15] * other.m[15],
    		]
    	}
    }
}

fn mul_matrix() {
	let mut mat = Matrix4x4{m: [
			1.0, 2.0, 3.0, 4.0,
			2.0, 3.0, 4.0, 5.0,
			3.0, 4.0, 5.0, 6.0,
			4.0, 5.0, 6.0, 7.0
		]};
	for _ in 0..10 {
		mat = mat * mat
	}
}

#[bench]
fn bench_mul_matrix(b: &mut Bencher) {
    b.iter(|| mul_matrix());
}

fn main() {
	let mat = Matrix4x4{m: [
			1.0, 2.0, 3.0, 4.0,
			2.0, 3.0, 4.0, 5.0,
			3.0, 4.0, 5.0, 6.0,
			4.0, 5.0, 6.0, 7.0
		]};
	println!("{:?}", mat*mat);
}