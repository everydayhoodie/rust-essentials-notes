fn main() {
    let n = 42u32;
    let n2 = n; // a copy of the value from n to n2
    life(n);
    // println!("{}", m); error: unresolved name `m`.
    // println!{"{}", o}; error: unresolved name `o`.

    {
        let phi = 1.618;
    }

    // println!("The value of phi is {}", phi); // is error

    let mag = MagicNumber { value: 42};
    let mag2 = mag;

    println!("{:?}", &mag as *const MagicNumber);
    println!("{:?}", &mag2 as *const MagicNumber);

    let mag3 = mag.clone();
    println!("{:?}", &mag3 as *const MagicNumber); // address is 0x7fff51e3d878
}

struct Magician {
    name: &'static str,
    power: u32
}

struct MagicNumbers<'a> {
    magn1: &'a u32,
    magn2: &'a u32
}

#[derive(Copy, Clone)]
struct MagicNumber {
    value: u64
}

// impl Copy for MagicNumber {}

fn life(m: u32) -> u32 {
    let o = m;
    o
}

// fn return_magician<'a>() -> &'a Magician {
//     let mag = Magician { name: "Gandalf", power: 4625 };
//     &mag // error: `mag` does not live long enough
// }
