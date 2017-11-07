mod math;
use math::*;

mod particle;
use particle::*;

mod gravity;
use gravity::*;

mod inv_square;
use inv_square::*;


extern crate rand;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};

extern crate gnuplot;
use gnuplot::*;

const ME : f64 = 5.972e24;
const RE : f64 = 6.371e6;
const STEPSIZE: f64 = 0.000001;
const PRINTSIZE: f64 = 100.0 * STEPSIZE;

fn main() {
	
	let mut objs = vec![Particle::at_rest("earth", ME, vec![0.0,0.0,0.0])];
	let range = Range::new(10.0, 100.0);
	let mut rng = rand::thread_rng();
	
	let mut t_hist = Vec::new();
	let mut gpe_hist = Vec::new();
	let mut ke_hist = Vec::new();

	let mut numname = String::new();
	for i in 0..1 {
		numname = i.to_string().to_owned();
		objs.push( 
			Particle::at_rest(
				&numname.clone(),
				10.0, //range.ind_sample(&mut rng) * ME, 
				vec![RE+ 10.0, 0.0,0.0]//scmultv(&(1.0/RE), &vec![range.ind_sample(&mut rng), range.ind_sample(&mut rng), range.ind_sample(&mut rng)])
			)
		);
	}
	
	let basegpe = -1.0 * G * objs[0].mass * objs[1].mass * 1.0/RE.powi(2); 	

	let mut t = 0.0;

	let mut gravgraph = Figure::new();
	let mut kegraph = Figure::new();

	println!("{}", 10.0*9.8*10.0);
	loop {
		if t > 1.428{
			println!("{}", 0.5*objs[1].mass*mag(&objs[1].vel).powi(2));
			break;
		}
		t += STEPSIZE;
		gravity::Gravity::sim_step_dbg(&mut objs, STEPSIZE, false);
		if t % PRINTSIZE < STEPSIZE {
			t_hist.push(t);
			
			let gpe : f64 = -1.0 * G * objs[0].mass * objs[1].mass *1.0/mag(&vminus(&objs[0].pos, &objs[1].pos));
			gpe_hist.push(objs[1].mass * 9.82 * (objs[1].pos[0] - RE));
			gravgraph.clear_axes();
			gravgraph.axes2d()
				.set_x_range(Fix(0.0), Fix(t))
				.set_y_range(Fix(0.0), Fix(10.0*10.0*9.82))
				.lines(t_hist.iter(), gpe_hist.iter(), &[]);
			gravgraph.show();

			let ke = 0.5 * objs[1].mass * mag(&objs[1].vel).powi(2);
			ke_hist.push(ke);
			kegraph.clear_axes();
			kegraph.axes2d()
				.set_x_range(Fix(0.0), Fix(t))
				.set_y_range(Fix(0.0), Fix(10.0*9.8*10.0))
				.lines(t_hist.iter(), ke_hist.iter(), &[]);
			kegraph.show();
		}
	}
}

fn e_total(p : &Particle, objs : &Vec<Particle>) -> f64 {
	gpe_total(p, objs) + p.ke() 
}

fn gpe_total(p : &Particle, objs : &Vec<Particle>) -> f64 {
	let gravpe : f64 = objs.iter()
		.filter(|a| *a as *const Particle !=  p as *const Particle)
		.map(|a| Gravity::gravpe(&a, &p))
		.sum();
	gravpe
}
