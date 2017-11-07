
use particle::*;

use math::*;

use inv_square::InvSquare;

pub const G : f64 = 6.674e-11;

pub struct Gravity {}

impl Gravity {
    pub fn gravpe(a : &Particle, b : &Particle) -> f64 {
        G*-1.0*a.mass*b.mass/mag(&vminus(&a.pos, &b.pos))
    }
}

impl InvSquare for Gravity {
    fn scale () -> f64 {
        G
    }

    fn value(p : &Particle) -> f64 {
        p.mass
    }

    fn tag() -> String {
        "Gravity".to_owned()
    }
}

