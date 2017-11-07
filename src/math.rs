use std::string::ToString;
use std::marker::Sized;

pub trait PhysVect where Self: Sized{
    fn plus(&self, other : &Self) -> Self;

    fn minus(&self, other : &Self) -> Self;

    fn scale(&self, scalar : &f64) -> Self;

    fn mag(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn unit(&self) -> Self {
        self.scale(&(1.0/self.mag()))
    }

    fn dot(&self, other : &Self) -> f64;
}

impl PhysVect for Vec<f64>{
    fn plus(&self, other : &Vec<f64>) -> Vec<f64> {
        self.iter().zip(other.iter())
            .map(|(a, b)| a + b)
            .collect()
    }

    fn minus(&self, other : &Vec<f64>) -> Vec<f64> {
        self.iter().zip(other.iter())
            .map(|(a,b)| a-b)
            .collect()
    }

    fn scale(&self, scalar : &f64) -> Vec<f64> {
        self.iter()
            .map(|a| a * scalar)
            .collect()
    }

    fn dot(&self, other : &Vec<f64>) -> f64 {
        self.iter().zip(other.iter())
            .map(|(a, b)| a*b)
            .sum()
    }
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
