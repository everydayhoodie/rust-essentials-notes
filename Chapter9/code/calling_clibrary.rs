#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct Complex {
    re: f32,
    im: f32,
}

#[link(name = "m")]
extern {
    fn ctanf(z: Complex) -> Complex;
}

fn tan(z: Complex) -> Complex {
    unsafe { ctanf(z) }
}

fn main() {
    let z = Complex { re: -1., im: 1. }; // z is  -1 + i
    let z_tan = tan(z);
    println!("the tangens of {:?} is {:?}", z, z_tan);
}
