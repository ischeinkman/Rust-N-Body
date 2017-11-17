
use math::*; 
use std::fmt::{Display, Formatter, Result};

pub struct Particle {
	pub pos : RectPhysVect,
	pub vel : RectPhysVect,
	pub acc : RectPhysVect,
	pub mass : f64,
	pub charge : f64,
	pub name : String
}

pub fn center_of_mass(objs : &Vec<Particle>) -> RectPhysVect {
	let mut mass = 0.0;
	let mut weightpos = RectPhysVect { x : 0.0, y : 0.0, z : 0.0};
	for obj in objs {
		weightpos = weightpos.plus(&obj.pos.scale(&obj.mass));
		mass += obj.mass;
	}
	weightpos.scale(&(1.0/mass))
}

impl Particle {
	pub fn apply_force(&mut self, f : &RectPhysVect) {
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
		Particle {
			name : name.to_owned(),
			mass,
			pos : RectPhysVect::from_vect(&pos),
			vel : RectPhysVect {x : 0.0, y : 0.0, z : 0.0},
			acc : RectPhysVect {x : 0.0, y : 0.0, z : 0.0},
			charge : 0.0
		}
	}	
}

impl Display for Particle {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "Particle {}:\n Pos: {},\n Vel: {},\n Acc: {},\n Mass: {}\n\n", self.name, self.pos.to_string(), self.vel.to_string(), self.acc.to_string(), self.mass)
	}
}
