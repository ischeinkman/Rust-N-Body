use particle::*;
use math::*;

pub trait InvSquare {
    fn scale() -> f64;

    fn value(p : &Particle) -> f64;

    fn tag() -> String;

    fn force_mag(p1 : &Particle, p2 : &Particle) -> f64 {
        Self::scale() * Self::value(p1) * Self::value(p2) / mag(&vminus(&p1.pos, &p2.pos)).powi(2)
    }

    fn force_vec(a : &Particle, b : &Particle) -> Vec<f64> {
        let mag = Self::force_mag(&a, &b);
        let dir = scmultv(&-1.0, &norm(&vminus(&a.pos, &b.pos)));
        scmultv(&mag, &dir)
    }

    fn sim_step(objs : &mut Vec<Particle>, step : f64) {
        Self::sim_step_dbg(objs, step, false)
    }

    fn sim_step_dbg(objs : &mut Vec<Particle>, step : f64, debug : bool) {
        let objslen = objs.len();
        for i in 0..objslen-1 {
            let (rna, rnb) = objs.split_at_mut(i+1);
            for j in i+1..objslen {
                let obi : &mut Particle = &mut rna[i];
                let obb : &mut Particle = &mut rnb[j-i-1];
                let gf = Self::force_vec(&obi, &obb);
                obi.apply_force(&gf);
                obb.apply_force(&scmultv(&-1.0, &gf));
                if debug {
                    println!("Force: {}\nP1: {}\nP2: {}\n\n", vprint(&gf), &obi, &obb);
                }
            }
        }
        for obj in objs {
            obj.prog(&step);
        }
    }
}