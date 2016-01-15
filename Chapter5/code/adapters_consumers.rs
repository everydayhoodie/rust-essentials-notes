fn main() {
    let rng = 0..1000;
    let rngvec = rng.collect::<Vec<i32>>();
    // let rngvec = rng.collect::<Vec<_>>();
    // let rngvec: Vec<i32> = rng.collect();
    println!("{:?}", rngvec);

    let mut rng = 0..1000;
    let forty_two = rng.find(|n| *n >= 42);
    println!("{:?}", forty_two);

    let rng = 0..1000;
    let rng_even = rng.filter(|n| is_even(*n)).collect::<Vec<i32>>();
    println!("{:?}", rng_even);

    let rng = 0..1000;
    let rng_even_pow3 = rng.filter(|n| is_even(*n))
                            .map(|n| n * n * n )
                            .collect::<Vec<i32>>();
    println!("{:?}", rng_even_pow3);

    let rng = 0..1000;
    let rng_even_pow3_first5 = rng.filter(|n| is_even(*n))
                            .map(|n| n * n * n )
                            .take(5)
                            .collect::<Vec<i32>>();
    println!("{:?}", rng_even_pow3_first5);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
