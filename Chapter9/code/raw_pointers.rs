fn main() {
    let p_raw: *const u32 = &10;
    // let n = *p_raw; // compiler error
    unsafe {
        let n = *p_raw;
        println!("{}", n);
    }

    let gr: f32 = 1.618;
    let p_imm: *const f32 = &gr as *const f32; // explicit cast
    let mut m: f32 = 3.14;
    let p_mut: *mut f32 = &mut m; // implicit cast

    unsafe {
        let ref_imm: &f32 = &*p_imm;
        let ref_mut: &mut f32 = &mut *p_mut;
    }
}
