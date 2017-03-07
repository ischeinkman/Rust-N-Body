use std::string::ToString;

pub fn vplus(a : &Vec<f64>, b : &Vec<f64>) -> Vec<f64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| a + b) 
		.collect()
}


pub fn vminus( a : &Vec<f64>, b : &Vec<f64>) -> Vec<f64>{
	a.iter().zip(b.iter())
		.map(|(a,b)| a-b)
		.collect()
}

pub fn scmultv(k : &f64, v : &Vec<f64>) -> Vec<f64> {
	v.iter()
		.map(|a| a * k)
		.collect()
}

pub fn vplusi(a : &Vec<i64>, b : &Vec<i64>) -> Vec<i64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| a+b) 
		.collect()
}

pub fn vminusi(a : &Vec<i64>, b : &Vec<i64>) -> Vec<i64>{
	a.iter().zip(b.iter())
		.map(|(a, b)| a-b) 
		.collect()
}

pub fn scmultvi(k : &i64, v : &Vec<i64>) -> Vec<i64> {
	v.iter()
		.map(|a| a * k)
		.collect()
}


pub fn mag(v : &Vec<f64>) -> f64 {
	let sm : f64 = v.iter()
		.map(|a| a.powi(2))
		.sum();
	sm.sqrt()
}

pub fn norm(v : &Vec<f64>) -> Vec<f64> {
	scmultv(&(1.0/mag(v)), &v)
}

pub fn dot(a : &Vec<f64>, b : &Vec<f64>) -> f64 {
	a.iter().zip(b.iter())
		.map(|(a, b)| a*b) 
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
