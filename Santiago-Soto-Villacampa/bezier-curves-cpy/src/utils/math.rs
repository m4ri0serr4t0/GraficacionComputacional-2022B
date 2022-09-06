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