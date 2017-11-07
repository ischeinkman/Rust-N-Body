
use math::*; 
use std::fmt::{Display, Formatter, Result};

pub struct Particle {
	pub pos : Vec<f64>,
	pub vel : Vec<f64>,
	pub acc : Vec<f64>,
	pub mass : f64,
	pub charge : f64,
	pub name : String
}

pub fn center_of_mass(objs : &Vec<Particle>) -> Vec<f64> {
	let mut mass = 0.0;
	let mut weightpos = vec![0.0; objs[0].pos.len()];
	for obj in objs {
		weightpos = weightpos.plus(&obj.pos.scale(&obj.mass));
		mass += obj.mass;
	}
	weightpos.scale(&(1.0/mass))
}

impl Particle {
	pub fn apply_force(&mut self, f : &Vec<f64>) {
		self.acc = self.acc.plus(&f.scale(&(1.0/self.mass)));
	}
	
	pub fn prog(&mut self, t : &f64) {
		let posdiff = self.vel.scale(t).plus(&self.acc.scale(&(0.5 *t*t)));
		self.pos = self.pos.plus(&posdiff);
		let veldiff = self.acc.scale(&t);
		self.vel = self.vel.plus(&veldiff);
		
		//Zero out the acceleration; we recalculate them each step.
		self.acc = self.acc.scale(&0.0);
	}
	
	pub fn ke(&self) -> f64 {
		let v = self.vel.mag();
		0.5*self.mass*v*v
	}
	
	pub fn at_rest(name : &str, mass : f64, pos : Vec<f64>) -> Particle {
		
		let vel = vec![0.0,0.0,0.0];
		let acc = vec![0.0,0.0,0.0];
		Particle {
			name : name.to_owned(),
			mass,
			pos,
			vel,
			acc,
			charge : 0.0
		}
	}	
}

impl Display for Particle {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Particle {}:\n Pos: {},\n Vel: {},\n Acc: {},\n Mass: {}\n\n", self.name, vprint(&self.pos), vprint(&self.vel), vprint(&self.acc), self.mass)	
	}
}
