
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
		weightpos = vplus(&weightpos, &scmultv(&obj.mass, &obj.pos));
		mass += obj.mass;
	}
	scmultv(&(1.0/mass), &weightpos)
}

impl Particle {
	pub fn apply_force(&mut self, f : &Vec<f64>) {
		self.acc = vplus(&self.acc, &scmultv(&(1.0/self.mass), &f));
	}
	
	pub fn prog(&mut self, t : &f64) {
		let posdiff = vplus(&scmultv(t, &self.vel), &scmultv(&(0.5*t*t), &self.acc));
		self.pos = vplus(&self.pos, &posdiff);
		let veldiff = scmultv(t, &self.acc);
		self.vel = vplus(&self.vel, &veldiff);
		
		//Zero out the acceleration; we recalculate them each step.
		self.acc = scmultv(&0.0, &self.acc);
	}
	
	pub fn ke(&self) -> f64 {
		let v = mag(&self.vel);
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
