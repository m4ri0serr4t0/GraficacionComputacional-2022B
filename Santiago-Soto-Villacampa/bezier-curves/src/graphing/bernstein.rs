use crate::utils::math;

pub fn bern(i:i32, n:i32, u:f64) -> f64 {
	(math::fac(n) as f64 / ((math::fac(i) as f64) 
	* (math::fac(n - i) as f64))) 
	* math::pow(1.0 - u, n - i) 
	* math::pow(u, i)
}