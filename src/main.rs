mod math;
use math::*;

mod particle;
use particle::*;

mod gravity;
use gravity::*;

extern crate rand;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};

const me : f64 = 5.972e24;
const re : f64 = 6.371e6;

fn main() {
	let stepsize = 0.0010;
	let mut objs = vec![Particle::at_rest("earth", me, vec![0.0,0.0,0.0])];
	let range = Range::new(0.5, 1.0e7);
	let mut rng = rand::thread_rng();
	
	let mut numname = String::new();
	for i in 0..1000 {
		numname = i.to_string().to_owned();
		objs.push( 
			Particle::at_rest(
				&numname.clone(),
				range.ind_sample(&mut rng) * me, 
				scmultv(&(1.0/re), &vec![range.ind_sample(&mut rng), range.ind_sample(&mut rng), range.ind_sample(&mut rng)])
			)
		);
	}

	let mut t = 0.0;
	loop {
		t += stepsize;
		gravsimstep(&mut objs, stepsize, false);
		if(true || t % 1.0 <= stepsize) {
			println!("{}: {:?}",t, scmultv(&re,&center_of_mass(&objs)));
		}
	}
}

fn gravsimstep(objs : &mut Vec<Particle>, step : f64, debug : bool) {
	let objslen = objs.len();
	for i in 0..objslen-1 {
		let (rna, rnb) = objs.split_at_mut(i+1);
		for j in i+1..objslen {
			let obi : &mut Particle = &mut rna[i];
			let obb : &mut Particle = &mut rnb[j-i-1];
			let gf = gravforce(&obi, &obb);
			obi.apply_force(&gf);
			obb.apply_force(&scmultv(&-1.0, &gf));
		}
	}
	for obj in objs {
		if(debug) {
			println!("{}", obj);
		}
		obj.prog(&step);	
	}
	if(debug) {
		println!("\n\n----------");
	}
}
