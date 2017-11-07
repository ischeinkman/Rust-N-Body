use particle::*;

use math::*;

use inv_square::InvSquare;

const E0 :f64 = 8.854187817;

pub struct Charge {}

impl InvSquare for Charge {
    fn value(p : &Particle) -> f64 {
        p.charge
    }
    fn tag() -> String {
        "Electric Field".to_owned()
    }
    fn scale() -> f64 {
        1/(4.0 * f64::consts::PI * E0)
    }
}

