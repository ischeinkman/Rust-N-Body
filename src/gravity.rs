
use particle::*;

use math::*;

const G : f64 = 6.674e-11;

pub fn gravmag(a : &Particle, b : &Particle) -> f64 {
	let dist = mag(&vminus(&a.pos, &b.pos));
	let distsq = dist*dist;
	let top = G*a.mass*b.mass;
	top/distsq
}

pub fn gravforce(a : &Particle, b : &Particle) -> Vec<f64> {
	let mag = gravmag(&a, &b);
	let dir = scmultv(&-1.0, &norm(&vminus(&a.pos, &b.pos)));
	scmultv(&mag, &dir)
}

pub fn gravpe(a : &Particle, b : &Particle) -> f64 {
	G*-1.0*a.mass*b.mass/mag(&vminus(&a.pos, &b.pos))
}
