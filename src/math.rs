use std::string::ToString;
use std::marker::Sized;

pub trait StandardVect where Self: Sized {
    fn plus(&self, other: &Self) -> Self;

    fn minus(&self, other: &Self) -> Self;

    fn scale(&self, scalar: &f64) -> Self;

    fn mag(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn unit(&self) -> Self {
        self.scale(&(1.0 / self.mag()))
    }

    fn dot(&self, other: &Self) -> f64;
}

pub trait CrossVect: StandardVect {
    fn cross(&self, other: &Self) -> Self;
}

pub trait PhysVect: CrossVect {
    fn to_rect(&self) -> RectPhysVect;
    fn from_rect(base : &RectPhysVect) -> Self;
}

pub fn vprint<T: ToString>(v: &Vec<T>) -> String {
    let mut rval = String::new();
    rval.push_str("(");
    for k in v {
        rval.push_str(&k.to_string());
        rval.push_str(", ");
    }
    rval.pop();
    rval.pop();
    rval.push_str(")");
    rval
}

pub struct RectPhysVect {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl RectPhysVect {
    pub fn from_vect(coords: &Vec<f64>) -> RectPhysVect {
        RectPhysVect { x: coords[0], y: coords[1], z: coords[2] }
    }
}

impl PhysVect for RectPhysVect {
    fn to_rect(&self) -> RectPhysVect {
        RectPhysVect {x : self.x, y : self.y, z : self.z}
    }
    fn from_rect(base : &RectPhysVect) -> Self {
        RectPhysVect {x : base.x, y : base.y, z : base.z}
    }
}

impl CrossVect for RectPhysVect {
    fn cross(&self, other: &RectPhysVect) -> RectPhysVect {
        RectPhysVect {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl StandardVect for RectPhysVect {
    fn plus(&self, other: &RectPhysVect) -> RectPhysVect {
        RectPhysVect {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn minus(&self, other: &RectPhysVect) -> RectPhysVect {
        RectPhysVect {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(&self, scalar: &f64) -> RectPhysVect {
        RectPhysVect {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }

    fn mag(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn unit(&self) -> RectPhysVect {
        self.scale(&(1.0 / self.mag()))
    }

    fn dot(&self, other: &RectPhysVect) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ToString for RectPhysVect {
    fn to_string(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }
}

