fn main() {
    let sum = (0..101).fold(0, |sum, n| sum + n);
    println!("{}", sum); // prints out 5050

    let prcub = (1..6).fold(1, |prcub, n| prcub * n * n * n);
    println!("{}", prcub); // prints out 1728000

    let acc = ([1, 9, 2, 3, 14, 12]).iter().fold(0, |acc, n| acc - n);
    println!("{}", acc); // prints out -41
}
