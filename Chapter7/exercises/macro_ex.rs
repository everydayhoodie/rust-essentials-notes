macro_rules! mac2 {
    ($arg:expr) => ( $arg * 3 );
}

macro_rules! mac3 {
    ($arg:ident) => (let $arg = 42 );
}

macro_rules! mac4 {
    ($arg:expr) => (
        println!("start - {} - end", $arg);
    );
}

fn main() {
    println!("{}", mac2!(5));
    println!("{}", mac2!(2 + 3));

    mac3!(x);
    println!("{}", x);

    mac4!("Where am I?");
}
