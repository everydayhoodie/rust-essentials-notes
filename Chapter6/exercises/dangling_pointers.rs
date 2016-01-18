fn main() {
    // let m: &u32 = {
    //     let n = &5u32;
    //     &*n
    // };
    // let o = *m;

    let mut x = &3;
    {
        let mut y = 4;
        // x = &y;
    }
}
