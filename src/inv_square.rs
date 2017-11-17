use particle::*;
use math::*;

pub trait InvSquare {
    fn scale() -> f64;

    fn value(p : &Particle) -> f64;

    fn tag() -> String;

    fn force_mag(p1 : &Particle, p2 : &Particle) -> f64 {
        Self::scale() * Self::value(p1) * Self::value(p2) /
            p1.pos.minus(&p2.pos).mag().powi(2)
    }

    fn force_vec(a : &Particle, b : &Particle) -> RectPhysVect {
        let mag = Self::force_mag(&a, &b);
        let dir = a.pos.minus(&b.pos).unit().scale(&-1.0);
        dir.scale(&mag)
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
                obb.apply_force(&gf.scale(&-1.0));
                if debug {
                    println!("Force: {}\nP1: {}\nP2: {}\n\n", gf.to_string(), &obi, &obb);
                }
            }
        }
        for obj in objs {
            obj.prog(&step);
        }
    }
}