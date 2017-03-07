
use math::*; 
use std::fmt::{Display, Formatter, Result};
use std::borrow::Cow;

pub struct Particle<'a> {
	pub pos : Vec<f64>,
	pub vel : Vec<f64>,
	pub acc : Vec<f64>,
	pub mass : f64,
	pub name : Cow<'a, str>,
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

impl <'a> Particle<'a> {
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
	
	pub fn kinrgy(&self) -> f64 {
		let v = mag(&self.vel);
		0.5*self.mass*v*v
	}
	
	pub fn at_rest(name : &str, mass : f64, pos : Vec<f64>) -> Particle<'a> {
		
		let vel = vec![0.0,0.0,0.0];
		let acc = vec![0.0,0.0,0.0];
		Particle {
			name : Cow::Owned(name.to_string()),
			mass : mass,
			pos : pos,
			vel : vel,
			acc : acc,
		}
	}	
}

impl <'a> Display for Particle<'a> {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Particle {}:\n Pos: {},\n Vel: {},\n Acc: {},\n Mass: {}\n\n", self.name, vprint(&self.pos), vprint(&self.vel), vprint(&self.acc), self.mass)	
	}
}
