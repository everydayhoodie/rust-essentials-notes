fn main() {
    let s1 = S1 { data: 5 };
    let s2 = S2 { data: 3.0 };

    draw_object(s1);
    draw_object(s2);
}

fn draw_object<T: Draw>(s: T) {
    s.draw();
}

struct S1 {
    data: i32
}

struct S2 {
    data: f32
}

trait Draw {
    fn draw(&self);
}

impl Draw for S1 {
    fn draw(&self) {
        println!("{}", self.data);
    }
}

impl Draw for S2 {
    fn draw(&self) {
        println!("{}", self.data);
    }
}
