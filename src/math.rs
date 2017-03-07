use std::string::ToString;

extern crate num;
use self::num::NumCast;
use self::num::cast::cast;


pub fn vplus<T : NumCast + Clone, K : NumCast + Clone>(a : &Vec<T>, b : &Vec<K>) -> Vec<f64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| cast::<T, f64>(a.clone()).unwrap() + cast::<K, f64>(b.clone()).unwrap())
		.collect()
}


pub fn vminus<T : NumCast + Clone, K : NumCast + Clone>(a : &Vec<T>, b : &Vec<K>) -> Vec<f64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| cast::<T, f64>(a.clone()).unwrap() - cast::<K, f64>(b.clone()).unwrap())
		.collect()
}

pub fn vplusi<T : NumCast + Clone, K : NumCast + Clone>(a : &Vec<T>, b : &Vec<K>) -> Vec<i64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| cast::<T, i64>(a.clone()).unwrap() + cast::<K, i64>(b.clone()).unwrap())
		.collect()
}


pub fn vminusi<T : NumCast + Clone, K : NumCast + Clone>(a : &Vec<T>, b : &Vec<K>) -> Vec<i64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| cast::<T, i64>(a.clone()).unwrap() - cast::<K, i64>(b.clone()).unwrap())
		.collect()
}

pub fn scmultvi<T: NumCast + Clone, K: NumCast + Clone>(k : &K, v : &Vec<T>) -> Vec<i64> {
	let ck : i64 = cast(k.clone()).unwrap();
	v.iter()
		.map(|a| cast::<T, i64>(a.clone()).unwrap())
		.map(|ca| ca * ck)
		.collect()
}

pub fn scmultv<T: NumCast + Clone, K: NumCast + Clone>(k : &K, v : &Vec<T>) -> Vec<f64> {
	let ck : f64 = cast(k.clone()).unwrap();
	v.iter()
		.map(|a| cast::<T, f64>(a.clone()).unwrap())
		.map(|ca| ca * ck)
		.collect()
}



pub fn mag<T : NumCast + Clone>(v : &Vec<T>) -> f64 {
	let sm : f64 = v.iter()
		.map(|a| cast::<T, f64>(a.clone()).unwrap())
		.map(|af| af.powi(2))
		.sum();
	sm.sqrt()
}

pub fn norm<T : NumCast + Clone>(v : &Vec<T>) -> Vec<f64> {
	scmultv(&(1.0/mag(v)), &v)
}

pub fn dot<T : NumCast + Clone, K : NumCast + Clone>(a : &Vec<T>, b : &Vec<K>) -> f64 {
	a.iter().zip(b.iter())
		.map(|(a, b)| cast::<T, f64>(a.clone()).unwrap() * cast::<K, f64>(b.clone()).unwrap())
		.sum()
}


pub fn vprint<T : ToString>(v : &Vec<T>) -> String {
	let mut rval = String::new();
	rval.push_str("(");
	for k in v {
		rval.push_str(&k.to_string());
		rval.push_str(", ");
	}
	rval.pop();
	rval.pop();
	rval.push_str(")");
	rval
}
