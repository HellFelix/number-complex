use std::{ops::{Add, Sub, Mul, Div}, fmt::{Display, Debug}, f64::consts::PI};

pub trait Complex: Add + Sub + Mul + Div + Display + Debug + Sized {
    fn conjugate(&self) -> Self;
}

 #[derive(Debug)]
pub struct Rectangular {
    real: f64,
    imag: f64,
}

impl Rectangular {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn get_polar(&self) -> Polar {
        let modulus = (self.real.powf(2.) + self.imag.powf(2.)).sqrt();
        let mut arg = (self.real/self.imag).atan();


        if self.real.is_sign_negative() && self.imag.is_sign_negative() {
            // arctan registers in first quadrant but we're actually in third
            arg += PI;
        } else if self.real.is_sign_positive() && self.imag.is_sign_negative() {
            // arctan registers in fourth quadrant but we're actually in second
            arg += PI
        } else if arg.is_sign_negative() {
            arg += 2.*PI;
        }

        Polar::new(arg, modulus)
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }
}

impl Complex for Rectangular {
    fn conjugate(&self) -> Self {
        Self { real: self.real, imag: -self.imag }
    }
}

impl Add for Rectangular {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imag + rhs.imag;
        Self { real, imag }
    }
}

impl Add<f64> for Rectangular {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Self { real, imag: self.imag }
    }
}

impl Sub for Rectangular {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let real = self.real - rhs.real;
        let imag = self.imag - rhs.imag;
        Self { real, imag }
    }
}

impl Sub<f64> for Rectangular {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self::Output {
        let real = self.real - rhs;
        Self { real, imag: self.imag }
    }
}

// we're going to have four terms, two of which are real and two of which are imaginary
// (a +bi)(c+ di) = ac + cbi + adi - bd
impl Mul for Rectangular {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let real = self.real * rhs.real - self.imag * rhs.imag;
        let imag = self.imag * rhs.real + self.real * rhs.imag;
        Self { real, imag }
    }
}

impl Mul<f64> for Rectangular {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let real = self.real * rhs;
        let imag = self.imag * rhs;
        Self { real, imag }
    }
}

// note that (a+bi)/(c+di) = (a+bi)(b-di)/(c^2 + d^2)
impl Div for Rectangular {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self * Self { real: rhs.real, imag: -rhs.imag };
        let denomenator = rhs.real.powf(2.) + rhs.imag.powf(2.);
        let real = numerator.real / denomenator;
        let imag = numerator.imag / denomenator;

        Self { real, imag }
    }
    
}

impl Div<f64> for Rectangular {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let real = self.real / rhs;
        let imag = self.real / rhs;
        Self { real, imag }
    }
}

impl Display for Rectangular {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
    
}

#[derive(Debug)]
pub struct Polar {
    arg: f64,
    modulus: f64,
}

impl Polar {
    pub fn new(arg: f64, modulus: f64) -> Self {
        Self { arg, modulus }
    }

    pub fn get_rectangular(&self) -> Rectangular {
        let real = self.modulus * self.arg.cos();
        let imag = self.modulus * self.arg.sin();
        Rectangular::new(real, imag)
    }

    pub fn arg(&self) -> f64 {
        self.arg
    }

    pub fn modulus(&self) -> f64 {
        self.modulus
    }
}

impl Complex for Polar {
    fn conjugate(&self) -> Self {
        Self { arg: 2.*PI - self.arg, modulus: self.modulus }
    }
}

impl Add for Polar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let rect_self = self.get_rectangular();
        let rect_rhs = rhs.get_rectangular();

        (rect_self + rect_rhs).get_polar()
    }
}

impl Sub for Polar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let rect_self = self.get_rectangular();
        let rect_rhs = rhs.get_rectangular();

        (rect_self - rect_rhs).get_polar()
    }
}

impl Mul for Polar {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let modulus = self.modulus * rhs.modulus;
        let mut arg = self.arg + rhs.arg;

        if arg > 2.*PI {
            arg -= 2.* PI;
        }

        Self { arg, modulus }
    }
}

impl Div for Polar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let modulus = self.modulus / rhs.modulus;
        let mut arg = self.arg - rhs.arg;

        if arg < 0. {
            arg += 2.*PI;
        }

        Self { arg, modulus }
    }
}

impl Display for Polar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}*e^{}i", self.modulus, self.arg)
    }
}
