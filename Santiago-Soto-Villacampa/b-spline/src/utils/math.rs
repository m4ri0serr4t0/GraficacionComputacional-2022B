use std::fmt;

pub fn fac(n:i32) -> i32 {
	if n <= 1 {
		1
	} else {
		n * fac(n - 1)
	}
}

pub fn pow(x:f64, exp:i32) -> f64 {
	let mut num:f64 = 1.0;

	for _i in 0..exp {
		num = num * x;
	}
	num
}

pub trait Float {
	fn precision(&self, digits:i32) -> f64;
}

impl Float for f64 {
	fn precision(&self, digits:i32) -> f64 {
		let str:String = format!("{0:.1$}", self, digits as usize);

		return str.parse().expect("Argumento invalido");
	}
}