
use particle::*;

use math::*;

pub const G : f64 = 6.674e-11;

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

pub fn grav_sim_step_dbg(objs : &mut Vec<Particle>, step : f64, debug : bool) {
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

pub fn grav_sim_step(objs : &mut Vec<Particle>, step : f64) {
	grav_sim_step_dbg(objs, step, false);
}
