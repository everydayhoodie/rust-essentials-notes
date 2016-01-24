use std::mem;
fn main() {
    let v: &[u8] = unsafe {
        mem::transmute("Gandalf")
    };
    println!("{:?}", v);
}
