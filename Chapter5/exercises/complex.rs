use std::f64;

fn main() {
    let mut c1 = Complex::new(2.0, 5.0);
    let c2 = Complex::new(3.0, 2.0);
    let mut c3 = Complex::new(2.0, 5.0);
    let c4 = Complex::new(2.0, 5.0);

    println!("{}", c1.to_string());  // 2 + 5i
    println!("{}", c1.add(c2).to_string());  // 5 + 7i

    c3.times_ten();
    println!("{}", c3.to_string());  // 20 + 50i
    println!("{}", c4.abs().to_string()); // 5.385164807134504
}

struct Complex {
    real: f64,
    imaginary: f64
}

impl Complex {
    fn new(r: f64, i: f64) -> Complex {
        Complex{ real: r, imaginary: i}
    }

    fn to_string(&self) -> String {
        if self.imaginary > 0.0 {
            format!("{} + {}i", self.real, self.imaginary)
        } else if self.imaginary < 0.0 {
            format!("{} - {}i", self.real, f64::abs(self.imaginary))
        } else {
            format!("{}", self.real)
        }
    }

    fn add(&mut self, other: Complex) -> Complex {
        Complex{ real: self.real + other.real, imaginary: self.imaginary + other.imaginary}
    }

    fn times_ten(&mut self) {
        self.real *= 10.0;
        self.imaginary *= 10.0;
    }

    fn abs(&self) -> f64 {
        f64::sqrt(self.real.powi(2) + self.imaginary.powi(2))
    }
}
