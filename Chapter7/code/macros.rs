macro_rules! welcome {
    () => (
        println!("Welcome to the Game!");
    )
}

macro_rules! mac1 {
    ($arg:expr) => (println!("arg is {}", $arg));
}

macro_rules! create_fn {
    ($fname:ident) => (
        fn $fname() {
            println!("Called the function {:?}", stringify!($fname));
        }
    );
}

macro_rules! massert {
    ($arg:expr) => (
        if $arg {}
        else { panic!("Assertion failed!"); }
    );
}

macro_rules! test_eq {
    ($name:ident, $left:expr, $right:expr) => {
        #[test]
        fn $name() {
            assert_eq!($left, $right);
        }
    };
}

test_eq!(seven_times_six_is_not_forty_two, 7 * 6, 42);
test_eq!(seven_times_six_is_not_forty_three, 7 * 6, 43);

fn main() {
    welcome!();
    mac1!(42);
    create_fn!(fn1);

    // massert!(1 == 42);
}
